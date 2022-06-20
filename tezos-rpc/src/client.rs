use std::result::Result;
use async_trait::async_trait;
use serde::Serialize;
use serde::{de::DeserializeOwned};
use tezos_core::types::encoded::{ChainID, Encoded};

use crate::error::{Error};
use crate::{constants};
use crate::models::checkpoint::{Checkpoint};
use crate::rpc_traits::{ShellRPC, ActiveRPC};

pub struct RpcContext {
    pub chain_alias: String,
}
impl RpcContext {
    /// Changes the chain alias used for requests under the `/chains/<chain_alias>/...` endpoint.
    pub fn change_chain_alias(&mut self, chain_alias: &str) {
        self.chain_alias = chain_alias.to_string()
    }
}

pub struct TezosRpc<'a> {
    pub node_url: &'a str,
    pub context: RpcContext,
    http: reqwest::Client,
}

impl<'a> TezosRpc<'a> {
    /// Creates a Tezos RPC client that will connect with the node specified with the node_url.
    ///
    /// ```rust
    /// use tezos_rpc::client::{TezosRpc};
    ///
    /// let client = TezosRpc::new("https://tezos-node.prod.gke.papers.tech");
    /// ```
    pub fn new(node_url: &'a str) -> Self {
        TezosRpc {
            node_url,
            context: RpcContext {
                chain_alias: constants::DEFAULT_CHAIN_ALIAS.to_string()
            },
            http: reqwest::Client::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.node_url, path)
    }

    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T, Error> {
        Ok(self.http.get(url).send().await?.json::<T>().await?)
    }

    async fn post<B: Serialize, T: DeserializeOwned>(&self, url: &String, body: &B) -> Result<T, Error> {
        Ok(self.http
            .post(url)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?)
    }
}

/// Implements protocol-independent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/shell/rpc.html) for more details.
#[async_trait]
impl<'a> ShellRPC for TezosRpc<'a> {
    /// Get the chain unique identifier.
    ///
    /// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
    async fn chain_id(&self) -> Result<ChainID, Error> {
        let path = format!("/chains/{}/chain_id", self.context.chain_alias);

        let chain_id = self.get(self.url(path.as_str())).await?;

        Ok(ChainID::new(chain_id)?)
    }

    /// Get the current checkpoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
    async fn checkpoint(&self) -> Result<Checkpoint, Error> {
        let path = format!("/chains/{}/levels/checkpoint", self.context.chain_alias);

        let checkpoint = self.get(self.url(path.as_str())).await?;

        Ok(checkpoint)
    }
}

/// Implements protocol-dependent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/active/rpc.html) for more details.
#[async_trait]
impl<'a> ActiveRPC for TezosRpc<'a> {
}

#[cfg(test)]
mod client_tests {
    use httpmock::prelude::*;
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
