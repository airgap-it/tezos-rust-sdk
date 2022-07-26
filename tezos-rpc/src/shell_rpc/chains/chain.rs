use crate::{client::TezosRpcChainId, http::Http};

pub mod blocks;
pub mod chain_id;
pub mod invalid_blocks;
pub mod is_bootstrapped;
pub mod levels;

use {crate::client::TezosRpcContext, crate::error::Error, serde::Serialize};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/{}", super::path(), chain_id.as_ref())
}

/// A builder to construct the properties of a request to forcefully set the bootstrapped flag of the node.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    payload: &'a PatchChainPayload,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, payload: &'a PatchChainPayload) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            payload,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(&self) -> Result<(), Error> {
        let path = self::path(self.chain_id.value());

        self.ctx
            .http_client()
            .patch::<_, serde_json::Value>(path.as_str(), Some(self.payload))
            .await?;

        Ok(())
    }
}

/// `PatchChainPayload` used in request [`PATCH /chains/<chain_id>`](patch)
#[derive(Serialize)]
pub struct PatchChainPayload {
    /// A chain identifier. This is either a chain hash in Ba&se58Check notation or a one the predefined aliases: 'main', 'test'.
    bootstrapped: bool,
}

/// Forcefully set the bootstrapped flag of the node.
///
/// [`PATCH /chains/<chain_id>`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
pub fn patch<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    payload: &'a PatchChainPayload,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, payload)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {crate::client::TezosRpc, crate::error::Error, httpmock::MockServer};

    #[tokio::test]
    async fn test_patch_chain() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(super::path(TezosRpcChainId::Main.value()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({}));
        });

        let client = TezosRpc::new(rpc_url);

        let req = super::PatchChainPayload {
            bootstrapped: false,
        };

        super::patch(&client.context(), &req).send().await
    }
}
