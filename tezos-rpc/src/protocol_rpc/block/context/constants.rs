use crate::{
    client::{TezosRpcChainId, TezosRpcContext},
    error::Error,
    http::Http,
    models::constants::Constants,
    protocol_rpc::block::BlockId,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/constants", super::path(chain_id, block_id))
}

/// A builder to construct the properties of a request to access the constants.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
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

    pub async fn send(&self) -> Result<Constants, Error> {
        let path = self::path(self.chain_id.value(), self.block_id);

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
    use crate::{
        client::{TezosRpc, TezosRpcChainId},
        error::Error,
        protocol_rpc::block::BlockId,
    };

    use httpmock::prelude::*;
    use num_bigint::BigInt;

    #[tokio::test]
    async fn test_get_genesis_constants() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
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

        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
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

        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!(
                    "constants/__TEST_DATA__/jakarta_constants.json"
                ));
        });
        let client = TezosRpc::new(rpc_url);

        let constants = client.get_constants().block_id(&block_id).send().await?;

        assert_eq!(
            constants.tokens_per_roll,
            Some(BigInt::from(6_000_000_000u64))
        );
        assert_eq!(constants.minimal_stake, None);
        assert_eq!(constants.tx_rollup_sunset_level, Some(3473409));
        assert_eq!(constants.time_between_blocks, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_lima_constants() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("constants/__TEST_DATA__/lima_constants.json"));
        });
        let client = TezosRpc::new(rpc_url);

        let constants = client.get_constants().block_id(&block_id).send().await?;

        assert_eq!(constants.tokens_per_roll, None);
        assert_eq!(
            constants.minimal_stake,
            Some(BigInt::from(6_000_000_000u64))
        );
        assert_eq!(constants.zk_rollup_enable, Some(false));
        assert_eq!(constants.zk_rollup_origination_size, Some(4_000));
        assert_eq!(constants.zk_rollup_min_pending_to_process, Some(10));

        Ok(())
    }
}
