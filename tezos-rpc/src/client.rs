use std::result::Result;
use async_trait::async_trait;
use tezos_core::types::encoded::{ChainID};

use crate::{http, shell_rpc};
use crate::error::{Error};
use crate::models::invalid_block::InvalidBlock;
use crate::{constants};
use crate::models::checkpoint::{Checkpoint};
use crate::shell_rpc::{ShellRPC};
use crate::active_rpc::{ActiveRPC};

pub struct RpcContext {
    pub chain_alias: String,
    pub http_client: http::TezosHttp,
}
impl RpcContext {
    /// Changes the chain alias used for requests under the `/chains/<chain_alias>/...` endpoint.
    pub fn change_chain_alias(&mut self, chain_alias: &str) {
        self.chain_alias = chain_alias.to_string()
    }
}

pub struct TezosRpc {
    pub context: RpcContext,
}

impl TezosRpc {
    /// Creates a Tezos RPC client that will connect with the node specified with the node_url.
    ///
    /// ```rust
    /// use tezos_rpc::client::{TezosRpc};
    ///
    /// let client = TezosRpc::new("https://tezos-node.prod.gke.papers.tech");
    /// ```
    pub fn new(rpc_endpoint: &str) -> Self {
        TezosRpc {
            context: RpcContext {
                chain_alias: constants::DEFAULT_CHAIN_ALIAS.to_string(),
                http_client: http::TezosHttp::new(rpc_endpoint),
            },
        }
    }
}

/// Implements protocol-independent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/shell/rpc.html) for more details.
#[async_trait]
impl<'a> ShellRPC for TezosRpc {
    /// Get the chain unique identifier.
    ///
    /// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
    async fn chain_id(&self) -> Result<ChainID, Error> {
        shell_rpc::chains::chain_id(&self.context).await
    }

    /// Get the current checkpoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
    async fn checkpoint(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::levels::checkpoint(&self.context).await
    }

    /// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
    ///
    /// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
    async fn invalid_blocks(&self) -> Result<Vec<InvalidBlock>, Error> {
        shell_rpc::chains::invalid_blocks(&self.context).await
    }
}

/// Implements protocol-dependent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/active/rpc.html) for more details.
#[async_trait]
impl<'a> ActiveRPC for TezosRpc {
}

#[cfg(test)]
mod client_tests {
    use httpmock::prelude::*;
    use tezos_core::types::encoded::Encoded;
    use super::*;
    use crate::{constants::{DEFAULT_CHAIN_ALIAS}};

    #[tokio::test]
    async fn test_change_chain_alias() -> Result<(), Error> {
        let mut client = TezosRpc::new("SOME_RPC");
        assert_eq!(client.context.chain_alias, DEFAULT_CHAIN_ALIAS);

        let new_chain_alias = "test";
        client.context.change_chain_alias(new_chain_alias);
        assert_eq!(client.context.chain_alias, new_chain_alias);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_chain_id() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(GET)
                .path("/chains/main/chain_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body("NetXdQprcVkpaWU");
        });

        let client = TezosRpc::new(rpc_url.as_str());
        let chain_id = client.chain_id().await?;
        assert_eq!("NetXdQprcVkpaWU", chain_id.base58());

        Ok(())
    }

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
                .path("/chains/main/invalid_blocks");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRpc::new(rpc_url.as_str());
        let response = client.invalid_blocks().await?;

        assert_eq!(response.len(), 1, "Expects a single invalid block.");

        let invalid_block = &response[0];
        assert_eq!(invalid_block.block, "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2");
        assert_eq!(invalid_block.level, 2424833);
        assert_eq!(invalid_block.errors.len(), 1, "Expects a single error.");
        assert_eq!(invalid_block.errors[0].kind, "permanent");
        assert_eq!(invalid_block.errors[0].id, "proto.alpha.Failed_to_get_script");
        assert_eq!(invalid_block.errors[0].contract, Some("KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_checkpoint() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!({
            "block_hash": "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2",
            "level": 2424833 as u64
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/chains/main/levels/checkpoint");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRpc::new(rpc_url.as_str());
        let response = client.checkpoint().await?;

        assert_eq!(response.block_hash, "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2");
        assert_eq!(response.level, 2424833);

        Ok(())
    }
}
