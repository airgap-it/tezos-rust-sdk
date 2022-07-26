use tezos_core::types::encoded::{BlockHash, Encoded};

use crate::{client::TezosRpcChainId, http::Http};

use {
    crate::client::TezosRpcContext, crate::error::Error, crate::models::invalid_block::InvalidBlock,
};

fn path<S: AsRef<str>>(chain_id: S, block_hash: S) -> String {
    format!("{}/{}", super::path(chain_id), block_hash.as_ref())
}

/// A builder to construct the properties of a request to get the errors that appeared during the block (in)validation.
#[derive(Clone, Copy)]
pub struct GetRPCRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_hash: &'a BlockHash,
}

impl<'a, HttpClient: Http> GetRPCRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, block_hash: &'a BlockHash) -> Self {
        GetRPCRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_hash,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(&self) -> Result<InvalidBlock, Error> {
        let path = self::path(self.chain_id.value(), self.block_hash.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// A builder to construct the properties of a request to get the errors that appeared during the block (in)validation.
#[derive(Clone, Copy)]
pub struct DeleteRPCRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_hash: &'a BlockHash,
}

impl<'a, HttpClient: Http> DeleteRPCRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, block_hash: &'a BlockHash) -> Self {
        DeleteRPCRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_hash,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(&self) -> Result<(), Error> {
        let path = self::path(self.chain_id.value(), self.block_hash.value());

        self.ctx
            .http_client()
            .delete::<(), serde_json::Value>(path.as_str(), None)
            .await?;

        Ok(())
    }
}

/// Get the errors that appeared during the block (in)validation.
///
/// [`GET /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks-block-hash)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    block_hash: &'a BlockHash,
) -> GetRPCRequestBuilder<'a, HttpClient> {
    GetRPCRequestBuilder::new(ctx, block_hash)
}

/// Remove an invalid block for the tezos storage.
///
/// [`DELETE <'a>/chains'a /<chain_id>/invalid_blocks/<bl'a ock_hash>`](htDeleteRPCRequestBuilder<'a>hell/rpc.html#delete-chains-chain-id-invalid-blocks-block-hash)
pub fn delete<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    block_hash: &'a BlockHash,
) -> DeleteRPCRequestBuilder<'a, HttpClient> {
    DeleteRPCRequestBuilder::new(ctx, block_hash)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use tezos_core::types::encoded::{BlockHash, Encoded};

    use crate::client::TezosRpcChainId;

    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_invalid_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let invalid_block_hash = "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2";
        let valid_response = serde_json::json!(
            {
                "block": invalid_block_hash,
                "level": 2424833 as u64,
                "errors": [
                    {
                        "kind": "permanent",
                        "id": "proto.alpha.Failed_to_get_script",
                        "contract": "KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW",
                    }
                ]
            }
        );

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                TezosRpcChainId::Main.value(),
                &invalid_block_hash.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRpc::new(rpc_url);
        let response = client
            .get_invalid_block(&invalid_block_hash.try_into().unwrap())
            .send()
            .await?;

        assert_eq!(
            response.block,
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
                .try_into()
                .unwrap()
        );
        assert_eq!(response.level, 2424833);
        assert_eq!(response.errors.len(), 1, "Expects a single error.");
        assert_eq!(response.errors[0].kind, "permanent");
        assert_eq!(response.errors[0].id, "proto.alpha.Failed_to_get_script");
        assert_eq!(
            response.errors[0].contract,
            Some("KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW".into())
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_invalid_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let invalid_block_hash: BlockHash = "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
            .try_into()
            .unwrap();

        server.mock(|when, then| {
            when.method(DELETE).path(super::path(
                TezosRpcChainId::Main.value(),
                invalid_block_hash.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({}));
        });

        let client = TezosRpc::new(rpc_url);

        client
            .remove_invalid_block(&invalid_block_hash)
            .send()
            .await
    }
}
