use crate::{client::TezosRpcChainId, http::Http};

use {
    crate::client::TezosRpcContext, crate::error::Error,
    crate::models::bootstrapped_status::BootstrappedStatus,
};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/is_bootstrapped", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the bootstrap status of a chain.
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

    pub async fn send(&self) -> Result<BootstrappedStatus, Error> {
        let path = self::path(self.chain_id.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Get the bootstrap status of a chain.
///
/// [`GET /chains/<chain_id>/is_bootstrapped`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-is-bootstrapped)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {
        crate::client::TezosRpc, crate::error::Error,
        crate::models::bootstrapped_status::ChainStatus, httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_is_bootstrapped() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!(
            {
                "bootstrapped": false,
                "sync_state": "stuck"
            }
        );

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });
        let client = TezosRpc::new(rpc_url);

        let response = client.is_bootstrapped().send().await?;

        assert_eq!(response.bootstrapped, false);
        assert_eq!(response.sync_state, ChainStatus::Stuck);

        Ok(())
    }
}
