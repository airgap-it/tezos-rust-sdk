use crate::{client::TezosRpcChainId, http::Http};

pub mod context;
pub mod hash;
pub mod helpers;

use {
    crate::models::block::BlockId,
    crate::{client::TezosRpcContext, error::Error, models::block::Block},
    derive_more::Display,
    serde::{Deserialize, Serialize},
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("/chains/{}/blocks/{}", chain_id.as_ref(), block_id.value())
}

/// A builder to construct the properties of a request to get all the information about a block.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    metadata: MetadataArg,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            metadata: MetadataArg::Always,
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

    /// Specify whether or not if the operations metadata should be returned.
    /// By default, the metadata will be returned depending on the node's metadata size limit policy.
    ///
    /// To get the metadata, even if it is needed to recompute them, use [MetadataArg::Always].
    ///
    /// To avoid getting the metadata, use [MetadataArg::Never].
    pub fn metadata(mut self, metadata: MetadataArg) -> Self {
        self.metadata = metadata;

        self
    }

    pub async fn send(&self) -> Result<Block, Error> {
        let path = self::path(self.chain_id.value(), self.block_id);

        let mut query: Vec<(&str, &'static str)> = vec![];

        // Add `metadata` query parameter
        query.push(("metadata", self.metadata.to_str()));

        self.ctx
            .http_client()
            .get_with_query(path.as_str(), &Some(query))
            .await
    }
}

/// * [MetadataArg::Always] - Block metadata is included in the response.
/// * [MetadataArg::Never] - Block metadata is not included in the response.
#[derive(Clone, Copy, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetadataArg {
    Always,
    Never,
}

impl MetadataArg {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
        }
    }
}

/// Get all the information about a block.
/// The associated metadata may not be present depending on the history mode and block's distance from the head.
///
/// Optional query arguments:
/// * `metadata` : Specifies whether or not if the operations metadata should be returned. To get the metadata, even if it is needed to recompute them, use `always`. To avoid getting the metadata, use `never`. By default, the metadata will be returned depending on the node's metadata size limit policy.
///
/// [`GET /chains/<chain_id>/blocks/<block_id>?[metadata=<metadata_rpc_arg>]`](https://tezos.gitlab.io/active/rpc.html#get-block-id)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use {
        super::*,
        crate::{client::TezosRpc, error::Error, models::block::TestChainStatusName},
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_genesis_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Genesis;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("block/__TEST_DATA__/block_genesis.json"));
        });
        let client = TezosRpc::new(rpc_url);

        let response = client
            .get_block()
            .block_id(&block_id)
            .metadata(super::MetadataArg::Always)
            .send()
            .await?;

        assert_eq!(
            response.protocol,
            "PrihK96nBAFSxVL1GLJTVhu9YnzkMFiBeuJRPA8NwuZVZCE1L6i"
                .try_into()
                .unwrap()
        );
        assert_eq!(response.chain_id, "NetXdQprcVkpaWU".try_into().unwrap());
        assert_eq!(
            response.hash,
            "BLockGenesisGenesisGenesisGenesisGenesisf79b5d1CoW2"
                .try_into()
                .unwrap()
        );
        assert_eq!(
            response.header.context,
            "CoV8SQumiVU9saiu3FVNeDNewJaJH8yWdsGF3WLdsRr2P9S7MzCj"
                .try_into()
                .unwrap()
        );

        let block_metadata = response.metadata.expect("Block has metadata");
        assert_eq!(
            block_metadata.protocol,
            "PrihK96nBAFSxVL1GLJTVhu9YnzkMFiBeuJRPA8NwuZVZCE1L6i"
                .try_into()
                .unwrap()
        );
        assert_eq!(block_metadata.baker, None);
        assert_eq!(
            block_metadata.test_chain_status.status,
            TestChainStatusName::NotRunning
        );

        assert_eq!(
            response.operations.len(),
            0,
            "No operations on genesis block."
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_2nd_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("block/__TEST_DATA__/block_1.json"));
        });
        let client = TezosRpc::new(rpc_url);

        client
            .get_block()
            .block_id(&block_id)
            .metadata(super::MetadataArg::Always)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_ithaca_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Level(2490368);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("block/__TEST_DATA__/block_ithaca.json"));
        });
        let client = TezosRpc::new(rpc_url);

        client
            .get_block()
            .block_id(&block_id)
            .metadata(super::MetadataArg::Always)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_jakarta_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Level(2504461);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(TezosRpcChainId::Main.value(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("block/__TEST_DATA__/block_jakarta.json"));
        });
        let client = TezosRpc::new(rpc_url);

        let block = client
            .get_block()
            .block_id(&block_id)
            .metadata(super::MetadataArg::Always)
            .send()
            .await?;

        assert_eq!(block.operations[3].last().unwrap().contents.len(), 6);

        Ok(())
    }
}
