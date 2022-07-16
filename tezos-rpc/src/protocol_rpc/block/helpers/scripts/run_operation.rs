use crate::http::Http;

use {
    crate::{
        client::TezosRPCContext,
        error::Error,
        models::operation::{OperationGroup, OperationWithMetadata},
        protocol_rpc::block::BlockID,
    },
    serde::Serialize,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID) -> String {
    format!("{}/run_operation", super::path(chain_id, block_id))
}

#[derive(Serialize)]
struct RunOperationParam<'a> {
    operation: &'a OperationGroup,
    chain_id: &'a str,
}

/// A builder to construct the properties of a request to run an operation without signature checks.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRPCContext<HttpClient>,
    chain_id: &'a str,
    block_id: &'a BlockID,
    operation: &'a OperationGroup,
}

impl<'a, HttpClient: Http> RPCRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRPCContext<HttpClient>, operation: &'a OperationGroup) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockID::Head,
            operation,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a str) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockID) -> &mut Self {
        self.block_id = block_id;

        self
    }

    pub async fn send(&self) -> Result<OperationWithMetadata, Error> {
        let path = self::path(self.chain_id, self.block_id);

        let param = RunOperationParam {
            operation: self.operation,
            chain_id: self.chain_id,
        };

        self.ctx
            .http_client()
            .post::<_, _, ()>(path.as_str(), &param, &None)
            .await
    }
}

/// Run an operation without signature checks.
///
/// [`POST /chains/<chain_id>/blocks/<block_id>/helpers/scripts/run_operation`](https://tezos.gitlab.io/api/rpc.html#post-block-id-helpers-scripts-run-operation)
pub fn post<'a, HttpClient: Http>(
    ctx: &'a TezosRPCContext<HttpClient>,
    operation: &'a OperationGroup,
) -> RPCRequestBuilder<'a, HttpClient> {
    RPCRequestBuilder::new(ctx, operation)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            client::TezosRPC,
            constants::DEFAULT_CHAIN_ALIAS,
            error::Error,
            models::operation::{
                kind::OperationKind, operation_contents_and_result::endorsement::Endorsement,
                OperationContent, OperationGroup, OperationWithMetadata,
            },
            protocol_rpc::block::BlockID,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_run_operation() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockID::Level(1);
        let operation_group = OperationGroup {
            protocol: Some("PtJakart2xVj7pYXJBXrqHgd82rdkLey5ZeeGwDgPp9rhQUbSqY".to_string()),
            branch: "BKoVxMrDZHW8yvh6u5pWwCS9qYi8ApUtjt5KMMBN5ofikNW1cJW".to_string(),
            contents: vec![
                OperationContent::Endorsement(
                    Endorsement {
                        kind: OperationKind::Endorsement,
                        slot: Some(0),
                        level: Some(2510083),
                        round: Some(0),
                        block_payload_hash: Some("vh32fG1tMNPtzZiKPHinfLPSAU3m2piFSgud4jBdaGSKJQH6q7Xd".to_string()),
                        metadata: None
                    }
                )
            ],
            signature: Some("sigqmbZ1v6kN6FC6L9aAZZkcrkF5NjmepPMqzn3FLW5PB31ERYPAy4ku4s865hY4eK4NGj6hjpR56W5GZquZKGQ9ibnFmtiR".to_string()),
            chain_id: None,
            hash: None,
        };
        let body = serde_json::to_string(&RunOperationParam {
            operation: &operation_group,
            chain_id: &DEFAULT_CHAIN_ALIAS.to_string(),
        })?;
        let response = serde_json::to_string(&OperationWithMetadata {
            contents: operation_group.contents.clone(),
            signature: operation_group.signature.clone(),
        })?;

        server.mock(|when, then| {
            when.method(POST)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id))
                .body(body);
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });
        let client = TezosRPC::new(rpc_url);

        let result = client
            .run_operation(&operation_group)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(result.signature, operation_group.signature);

        Ok(())
    }
}
