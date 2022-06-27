use std::result::Result;
use async_trait::async_trait;
use tezos_core::types::encoded::{ChainID, BlockHash};

use crate::shell_rpc::chains::chain::PatchChainRequest;
use crate::shell_rpc::chains::chain::blocks::GetBlocksQuery;
use crate::{http, shell_rpc};
use crate::error::{Error};
use crate::models::invalid_block::InvalidBlock;
use crate::{constants};
use crate::models::checkpoint::{Checkpoint};
use crate::shell_rpc::{ShellRPC};
use crate::active_rpc::{ActiveRPC};

pub struct TezosRPCContext {
    /// A chain identifier. This is either a chain hash in Base58Check notation or a one the predefined aliases: 'main', 'test'.
    pub chain_id: String,
    pub http_client: http::TezosHttp,
}
impl TezosRPCContext {
    /// Changes the rpc endpoint used in RPC requests.
    pub fn change_rpc_endpoint(&mut self, rpc_endpoint: &str) {
        self.http_client.change_rpc_endpoint(rpc_endpoint.to_string());
    }
}

pub struct TezosRPC {
    pub context: TezosRPCContext,
}

impl TezosRPC {
    /// Creates a Tezos RPC client that will connect to the specified node RPC.
    ///
    /// ```rust
    /// use tezos_rpc::client::{TezosRPC};
    ///
    /// let client = TezosRPC::new("https://tezos-node.prod.gke.papers.tech");
    /// ```
    pub fn new(rpc_endpoint: &str) -> Self {
        TezosRPC {
            context: TezosRPCContext {
                chain_id: constants::DEFAULT_CHAIN_ALIAS.to_string(),
                http_client: http::TezosHttp::new(rpc_endpoint),
            },
        }
    }

    /// Creates a Tezos RPC client that will connect to the specified node RPC.
    ///
    /// This method allows the user to provide the chain identifier that will be used when
    /// sending requests to the RPC. The default is `main`.
    ///
    /// ```rust
    /// use tezos_rpc::client::{TezosRPC};
    ///
    /// let client = TezosRPC::new_with_chain_id("https://tezos-node.prod.gke.papers.tech", "NetXLH1uAxK7CCh");
    /// ```
    pub fn new_with_chain_id(rpc_endpoint: &str, chain_id: &str) -> Self {
        TezosRPC {
            context: TezosRPCContext {
                chain_id: chain_id.to_string(),
                http_client: http::TezosHttp::new(rpc_endpoint),
            },
        }
    }
}

#[async_trait]
impl<'a> ShellRPC for TezosRPC {
    async fn patch_chain(&self, body: PatchChainRequest) -> Result<(), Error> {
        shell_rpc::chains::chain::patch(&self.context, body).await
    }

    async fn get_chain_id(&self) -> Result<ChainID, Error> {
        shell_rpc::chains::chain::chain_id::get(&self.context).await
    }

    async fn get_blocks(&self, query: &GetBlocksQuery) -> Result<Vec<Vec<BlockHash>>, Error> {
        shell_rpc::chains::chain::blocks::get(&self.context, query).await
    }

    async fn get_invalid_blocks(&self) -> Result<Vec<InvalidBlock>, Error> {
        shell_rpc::chains::chain::invalid_blocks::get(&self.context).await
    }

    async fn get_checkpoint(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::checkpoint::get(&self.context).await
    }
}

#[async_trait]
impl<'a> ActiveRPC for TezosRPC {
}

#[cfg(test)]
mod client_tests {
    use httpmock::prelude::*;
    use tezos_core::types::encoded::Encoded;
    use super::*;

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

        let client = TezosRPC::new(rpc_url.as_str());
        let chain_id = client.get_chain_id().await?;
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

        let client = TezosRPC::new(rpc_url.as_str());
        let response = client.get_invalid_blocks().await?;

        assert_eq!(response.len(), 1, "Expects a single invalid block.");

        let invalid_block = &response[0];
        assert_eq!(invalid_block.block.base58(), "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2");
        assert_eq!(invalid_block.level, 2424833);
        assert_eq!(invalid_block.errors.len(), 1, "Expects a single error.");
        assert_eq!(invalid_block.errors[0].kind, "permanent");
        assert_eq!(invalid_block.errors[0].id, "proto.alpha.Failed_to_get_script");
        assert_eq!(invalid_block.errors[0].contract, Some("KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_blocks() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!(
            [
                [
                    "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8"
                ]
            ]
        );

        server.mock(|when, then| {
            when.method(GET)
                .path("/chains/main/blocks")
                .query_param("length", "1")
                .query_param("head", "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8")
                .query_param("min_date", "1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRPC::new(rpc_url.as_str());

        let req_query = &GetBlocksQuery{
            length: Some(1),
            head: Some(BlockHash::new("BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8".to_string())?),
            min_date: Some(1)
        };
        let response = client.get_blocks(req_query).await?;

        assert_eq!(response[0][0].base58(), "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8");

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

        let client = TezosRPC::new(rpc_url.as_str());
        let response = client.get_checkpoint().await?;

        assert_eq!(response.block_hash.base58(), "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2");
        assert_eq!(response.level, 2424833);

        Ok(())
    }
}
