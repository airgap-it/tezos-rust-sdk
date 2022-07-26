use crate::{client::TezosRpcChainId, http::Http};

use {
    crate::{
        client::TezosRpcContext,
        error::Error,
        models::operation::{Operation, OperationWithMetadata},
        protocol_rpc::block::BlockId,
    },
    serde::Serialize,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/run_operation", super::path(chain_id, block_id))
}

#[derive(Serialize)]
struct RunOperationParam<'a> {
    operation: &'a Operation,
    chain_id: &'a str,
}

/// A builder to construct the properties of a request to run an operation without signature checks.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    operation: &'a Operation,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, operation: &'a Operation) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            operation,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(mut self, block_id: &'a BlockId) -> Self {
        self.block_id = block_id;

        self
    }

    pub async fn send(&self) -> Result<OperationWithMetadata, Error> {
        let path = self::path(self.chain_id.value(), self.block_id);

        let param = RunOperationParam {
            operation: self.operation,
            chain_id: self.chain_id.chain_id_value(),
        };

        self.ctx
            .http_client()
            .post::<_, _, ()>(path.as_str(), &param, None)
            .await
    }
}

/// Run an operation without signature checks.
///
/// [`POST /chains/<chain_id>/blocks/<block_id>/helpers/scripts/run_operation`](https://tezos.gitlab.io/api/rpc.html#post-block-id-helpers-scripts-run-operation)
pub fn post<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    operation: &'a Operation,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, operation)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use {
        super::*,
        crate::{
            client::TezosRpc,
            error::Error,
            models::operation::{
                kind::OperationKind, operation_contents_and_result::endorsement::Endorsement,
                Operation, OperationContent, OperationWithMetadata,
            },
            protocol_rpc::block::BlockId,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_run_operation() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Level(1);
        let operation_group = Operation {
            protocol: Some("PtJakart2xVj7pYXJBXrqHgd82rdkLey5ZeeGwDgPp9rhQUbSqY".try_into().unwrap()),
            branch: "BKoVxMrDZHW8yvh6u5pWwCS9qYi8ApUtjt5KMMBN5ofikNW1cJW".try_into().unwrap(),
            contents: vec![
                OperationContent::Endorsement(
                    Endorsement {
                        kind: OperationKind::Endorsement,
                        slot: Some(0),
                        level: Some(2510083),
                        round: Some(0),
                        block_payload_hash: Some("vh32fG1tMNPtzZiKPHinfLPSAU3m2piFSgud4jBdaGSKJQH6q7Xd".try_into().unwrap()),
                        metadata: None
                    }
                )
            ],
            signature: Some("sigqmbZ1v6kN6FC6L9aAZZkcrkF5NjmepPMqzn3FLW5PB31ERYPAy4ku4s865hY4eK4NGj6hjpR56W5GZquZKGQ9ibnFmtiR".try_into().unwrap()),
            chain_id: None,
            hash: None,
        };
        let body = serde_json::to_string(&RunOperationParam {
            operation: &operation_group,
            chain_id: TezosRpcChainId::Main.chain_id_value(),
        })?;
        let response = serde_json::to_string(&OperationWithMetadata {
            contents: operation_group.contents.clone(),
            signature: operation_group.signature.clone(),
        })?;

        server.mock(|when, then| {
            when.method(POST)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id))
                .body(body);
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });
        let client = TezosRpc::new(rpc_url);

        let result = client
            .run_operation(&operation_group)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(result.signature, operation_group.signature);

        Ok(())
    }
}
