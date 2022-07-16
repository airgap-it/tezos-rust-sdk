use crate::{
    client::TezosRpcContext, error::Error, http::Http, models::constants::Constants,
    protocol_rpc::block::BlockID,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID) -> String {
    format!("{}/constants", super::path(chain_id, block_id))
}

/// A builder to construct the properties of a request to access the constants.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a str,
    block_id: &'a BlockID,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockID::Head,
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

    pub async fn send(&self) -> Result<Constants, Error> {
        let path = self::path(self.chain_id, self.block_id);

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Access the list of all constants.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/constants`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-constants)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::{constants::DEFAULT_CHAIN_ALIAS, protocol_rpc::block::BlockID};

    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_genesis_constants() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockID::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!(
                    "constants/__TEST_DATA__/block_1_constants.json"
                ));
        });
        let client = TezosRpc::new(rpc_url);

        let constants = client.get_constants().block_id(&block_id).send().await?;

        assert_eq!(constants.nonce_length, Some(32));
        assert_eq!(constants.time_between_blocks, Some(vec![60, 75]));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_ithaca_constants() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockID::Head;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!(
                    "constants/__TEST_DATA__/ithaca_constants.json"
                ));
        });
        let client = TezosRpc::new(rpc_url);

        let constants = client.get_constants().block_id(&block_id).send().await?;

        assert_eq!(constants.nonce_length, Some(32));
        assert_eq!(constants.tx_rollup_sunset_level, None);
        assert_eq!(constants.time_between_blocks, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_jakarta_constants() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockID::Head;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!(
                    "constants/__TEST_DATA__/jakarta_constants.json"
                ));
        });
        let client = TezosRpc::new(rpc_url);

        let constants = client.get_constants().block_id(&block_id).send().await?;

        assert_eq!(constants.tx_rollup_sunset_level, Some(3473409));
        assert_eq!(constants.time_between_blocks, None);

        Ok(())
    }
}
