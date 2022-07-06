use crate::{
    client::TezosRPCContext,
    error::Error,
    models::operation::{OperationGroup, OperationWithMetadata},
    protocol_rpc::block::BlockID,
};

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/operations", super::path(chain_id, block_id))
}

/// A builder to construct the properties of a request to access the constants.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a String,
    block_id: &'a BlockID,
    operations: &'a Vec<&'a OperationGroup>,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext, operations: &'a Vec<&OperationGroup>) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
            block_id: &BlockID::Head,
            operations,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a String) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockID) -> &mut Self {
        self.block_id = block_id;

        self
    }

    pub async fn send(self) -> Result<Vec<OperationWithMetadata>, Error> {
        let path = self::path(self.chain_id, self.block_id);

        self.ctx
            .http_client
            .post::<_, _, ()>(path.as_str(), self.operations, &None)
            .await
    }
}

/// Simulate the application of the operations with the context of the given block and return the result of each operation application.
///
/// [`POST /chains/<chain_id>/blocks/<block_id>/helpers/preapply/operations`](https://tezos.gitlab.io/active/rpc.html#post-block-id-helpers-preapply-operations)
pub fn post<'a>(
    ctx: &'a TezosRPCContext,
    operations: &'a Vec<&OperationGroup>,
) -> RPCRequestBuilder<'a> {
    RPCRequestBuilder::new(ctx, operations)
}

#[cfg(test)]
mod tests {
    use {
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
    async fn test_preapply_operations() -> Result<(), Error> {
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
        let body = serde_json::to_string(&vec![&operation_group])?;
        let response = serde_json::to_string(&vec![OperationWithMetadata {
            contents: operation_group.contents.clone(),
            signature: operation_group.signature.clone(),
        }])?;

        server.mock(|when, then| {
            when.method(POST)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id))
                .body(body);
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });
        let client = TezosRPC::new(rpc_url.as_str());

        let result = client
            .preapply_operations(&vec![&operation_group])
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(result[0].signature, operation_group.signature);

        Ok(())
    }
}
