use crate::{client::TezosRpcChainId, http::Http};

use {crate::client::TezosRpcContext, crate::error::Error};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/chain_id", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the chain unique identifier.
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

    pub async fn send(&self) -> Result<String, Error> {
        let path = self::path(self.chain_id.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Get the chain unique identifier.
///
/// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_chain_id() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let chain_id_string = "NetXdQprcVkpaWU";

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(chain_id_string);
        });

        let client = TezosRpc::new(rpc_url);
        let chain_id = client.get_chain_id().send().await?;
        assert_eq!(chain_id_string, chain_id);

        Ok(())
    }
}
