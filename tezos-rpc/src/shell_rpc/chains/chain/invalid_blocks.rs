use crate::{client::TezosRpcChainId, http::Http};

pub mod block;

use {
    crate::client::TezosRpcContext, crate::error::Error, crate::models::invalid_block::InvalidBlock,
};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/invalid_blocks", super::path(chain_id))
}

/// A builder to construct the properties of a request to get blocks that have been declared invalid.
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

    pub async fn send(&self) -> Result<Vec<InvalidBlock>, Error> {
        let path = self::path(self.chain_id.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
///
/// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_invalid_blocks() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!(
            [
                {
                    "block": "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2",
                    "level": 2424833 as u64,
                    "errors": [
                        {
                            "kind": "permanent",
                            "id": "proto.alpha.Failed_to_get_script",
                            "contract": "KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW",
                        }
                    ]
                }
            ]
        );

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRpc::new(rpc_url);
        let response = client.get_invalid_blocks().send().await?;

        assert_eq!(response.len(), 1, "Expects a single invalid block.");

        let invalid_block = &response[0];
        assert_eq!(
            invalid_block.block,
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
                .try_into()
                .unwrap()
        );
        assert_eq!(invalid_block.level, 2424833);
        assert_eq!(invalid_block.errors.len(), 1, "Expects a single error.");
        assert_eq!(invalid_block.errors[0].kind, "permanent");
        assert_eq!(
            invalid_block.errors[0].id,
            "proto.alpha.Failed_to_get_script"
        );
        assert_eq!(
            invalid_block.errors[0].contract,
            Some("KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW".into())
        );

        Ok(())
    }
}
