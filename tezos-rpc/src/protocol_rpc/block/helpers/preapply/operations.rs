use crate::{
    client::{TezosRpcChainId, TezosRpcContext},
    error::Error,
    http::Http,
    models::operation::{Operation, OperationWithMetadata},
    protocol_rpc::block::BlockId,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/operations", super::path(chain_id, block_id))
}

/// A builder to construct the properties of a request to simulate the application of the operations.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    operations: &'a Vec<&'a Operation>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, operations: &'a Vec<&Operation>) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            operations,
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

    pub async fn send(&self) -> Result<Vec<OperationWithMetadata>, Error> {
        let path = self::path(self.chain_id.value(), self.block_id);

        self.ctx
            .http_client()
            .post::<_, _, ()>(path.as_str(), self.operations, None)
            .await
    }
}

/// Simulate the application of the operations with the context of the given block and return the result of each operation application.
///
/// [`POST /chains/<chain_id>/blocks/<block_id>/helpers/preapply/operations`](https://tezos.gitlab.io/active/rpc.html#post-block-id-helpers-preapply-operations)
pub fn post<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    operations: &'a Vec<&Operation>,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, operations)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {
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
    async fn test_preapply_operations() -> Result<(), Error> {
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
        let body = serde_json::to_string(&vec![&operation_group])?;
        let response = serde_json::to_string(&vec![OperationWithMetadata {
            contents: operation_group.contents.clone(),
            signature: operation_group.signature.clone(),
        }])?;

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
            .preapply_operations(&vec![&operation_group])
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(result[0].signature, operation_group.signature);

        Ok(())
    }
}
