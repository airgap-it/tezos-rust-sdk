use crate::{client::TezosRpcChainId, http::Http};

use {crate::client::TezosRpcContext, crate::error::Error, crate::models::checkpoint::Checkpoint};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/caboose", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the current caboose for this chain.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(&self) -> Result<Checkpoint, Error> {
        let path = self::path(self.chain_id.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Get the current caboose for this chain.
///
/// [`GET /chains/<chain_id>/levels/caboose`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-caboose)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_caboose() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!({
            "block_hash": "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2",
            "level": 2424833 as u64
        });

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRpc::new(rpc_url);
        let response = client.get_caboose().send().await?;

        assert_eq!(
            response.block_hash,
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
                .try_into()
                .unwrap()
        );
        assert_eq!(response.level, 2424833);

        Ok(())
    }
}
