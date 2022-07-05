use {
    crate::client::TezosRPCContext, crate::error::Error, crate::protocol_rpc::block::BlockID,
    tezos_michelson::micheline::Micheline,
};

fn path(chain_id: &String, block_id: &BlockID, big_map_id: &u32, key: &String) -> String {
    format!("{}/{}", super::path(chain_id, block_id, big_map_id), key)
}

/// A builder to construct the properties of a request to access the value associated with a key in a big map.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a String,
    block_id: &'a BlockID,
    big_map_id: &'a u32,
    script_expr: &'a String,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext, big_map_id: &'a u32, script_expr: &'a String) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
            block_id: &BlockID::Head,
            big_map_id,
            script_expr,
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

    pub async fn send(self) -> Result<Micheline, Error> {
        let path = self::path(
            self.chain_id,
            self.block_id,
            self.big_map_id,
            self.script_expr,
        );

        self.ctx.http_client.get(path.as_str()).await
    }
}

/// Access the value associated with a key in a big map.
///
/// * `script_expr` - The Blake2b hash of the map key packed (Base58Check-encoded)
/// e.g. `expru3MJA26WX3kQ9WCPBPhCqsXE33BBtXnTQpYmQwtbJyHSu3ME9E`
///
/// [`GET /chains/<chain_id>/blocks/<block_id>/context/big_maps/<big_map_id>/<script_expr>`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-big-maps-big-map-id-script-expr)
pub fn get<'a>(
    ctx: &'a TezosRPCContext,
    big_map_id: &'a u32,
    script_expr: &'a String,
) -> RPCRequestBuilder<'a> {
    RPCRequestBuilder::new(ctx, big_map_id, script_expr)
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            client::TezosRPC, constants::DEFAULT_CHAIN_ALIAS, error::Error,
            protocol_rpc::block::BlockID,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_big_map_value() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let value = include_str!("__TEST_DATA__/michelson_value.json");
        let big_map_id: u32 = 162771;
        let big_map_key = "expru3MJA26WX3kQ9WCPBPhCqsXE33BBtXnTQpYmQwtbJyHSu3ME9E".to_string();
        let block_id = BlockID::Level(100);

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                &DEFAULT_CHAIN_ALIAS.to_string(),
                &block_id,
                &big_map_id,
                &big_map_key,
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(value);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let big_map = client
            .get_big_map_value(&big_map_id, &big_map_key)
            .block_id(&block_id)
            .send()
            .await?;

        assert!(big_map.is_micheline_primitive_application());

        Ok(())
    }
}
