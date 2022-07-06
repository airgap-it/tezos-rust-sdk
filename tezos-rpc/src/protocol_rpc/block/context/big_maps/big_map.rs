pub mod script_expr;

use {
    crate::client::TezosRPCContext, crate::error::Error, crate::protocol_rpc::block::BlockID,
    tezos_michelson::micheline::Micheline,
};

fn path(chain_id: &String, block_id: &BlockID, big_map_id: &u32) -> String {
    format!("{}/{}", super::path(chain_id, block_id), big_map_id)
}

/// A builder to construct the properties of a request to get the list of values in a big map
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a String,
    block_id: &'a BlockID,
    big_map_id: &'a u32,
    offset: Option<&'a u32>,
    length: Option<&'a u32>,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext, big_map_id: &'a u32) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
            block_id: &BlockID::Head,
            big_map_id,
            offset: None,
            length: None,
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

    /// Configure request to skip the first `offset` values. Useful in combination with `length` for pagination.
    pub fn offset(&mut self, offset: &'a u32) -> &mut Self {
        self.offset = Some(offset);

        self
    }

    /// Configure request to only retrieve `length` values. Useful in combination with `offset` for pagination.
    pub fn length(&mut self, length: &'a u32) -> &mut Self {
        self.length = Some(length);

        self
    }

    pub async fn send(self) -> Result<Micheline, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(offset) = self.offset {
            // Add `offset` query parameter
            query.push(("offset", offset.to_string()));
        }
        if let Some(length) = self.length {
            // Add `length` query parameter
            query.push(("length", length.to_string()));
        }

        let path = self::path(self.chain_id, self.block_id, self.big_map_id);

        self.ctx
            .http_client
            .get_with_query(path.as_str(), &Some(query))
            .await
    }
}

/// Get the (optionally paginated) list of values in a big map. Order of values is unspecified, but is guaranteed to be consistent.
///
/// Optional query arguments:
///
/// * `offset` : Skip the first `offset` values. Useful in combination with `length` for pagination.
/// * `length` : Only retrieve `length` values. Useful in combination with `offset` for pagination.
///
/// [`GET /chains/<chain_id>/blocks/<block_id>/context/big_maps/<big_map_id>?[offset=<uint>]&[length=<uint>]`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-big-maps-big-map-id)
pub fn get<'a>(ctx: &'a TezosRPCContext, big_map_id: &'a u32) -> RPCRequestBuilder<'a> {
    RPCRequestBuilder::new(ctx, big_map_id)
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
    async fn test_get_big_map() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let big_map = include_str!("__TEST_DATA__/mainnet_162771.json");
        let big_map_id: u32 = 162771;
        let block_id = BlockID::Level(100);

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                &DEFAULT_CHAIN_ALIAS.to_string(),
                &block_id,
                &big_map_id,
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(big_map);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let big_map = client
            .get_big_map(&big_map_id)
            .block_id(&block_id)
            .length(&100)
            .offset(&100)
            .send()
            .await?;

        assert!(big_map.is_micheline_sequence());

        Ok(())
    }
}
