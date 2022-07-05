pub mod context;

use {
    crate::models::block::BlockID,
    crate::{client::TezosRPCContext, error::Error, models::block::Block},
    derive_more::Display,
    serde::{Deserialize, Serialize},
};

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("/chains/{}/blocks/{}", chain_id, block_id.value())
}

/// A builder to construct the properties of a request to get all the information about a block.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a String,
    block_id: &'a BlockID,
    metadata: MetadataArg,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
            block_id: &BlockID::Head,
            metadata: MetadataArg::Always,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a String) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockID) -> &mut Self {
        self.block_id = block_id;

        self
    }

    /// Specify whether or not if the operations metadata should be returned.
    /// By default, the metadata will be returned depending on the node's metadata size limit policy.
    ///
    /// To get the metadata, even if it is needed to recompute them, use [MetadataArg::Always].
    ///
    /// To avoid getting the metadata, use [MetadataArg::Never].
    pub fn metadata(&mut self, metadata: MetadataArg) -> &mut Self {
        self.metadata = metadata;

        self
    }

    pub async fn send(self) -> Result<Block, Error> {
        let path = self::path(self.chain_id, self.block_id);

        let mut query: Vec<(&str, String)> = vec![];

        // Add `metadata` query parameter
        query.push(("metadata", self.metadata.to_string()));

        self.ctx
            .http_client
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

/// Get all the information about a block.
/// The associated metadata may not be present depending on the history mode and block's distance from the head.
///
/// Optional query arguments:
/// * `metadata` : Specifies whether or not if the operations metadata should be returned. To get the metadata, even if it is needed to recompute them, use `always`. To avoid getting the metadata, use `never`. By default, the metadata will be returned depending on the node's metadata size limit policy.
///
/// [`GET /chains/<chain_id>/blocks/<block_id>?[metadata=<metadata_rpc_arg>]`](https://tezos.gitlab.io/active/rpc.html#get-block-id)
pub fn get(ctx: &TezosRPCContext) -> RPCRequestBuilder {
    RPCRequestBuilder::new(ctx)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            client::TezosRPC, constants::DEFAULT_CHAIN_ALIAS, error::Error,
            models::block::TestChainStatusName,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_genesis_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockID::Genesis;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_genesis.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

        let response = client
            .get_block()
            .block_id(&block_id)
            .metadata(super::MetadataArg::Always)
            .send()
            .await?;

        assert_eq!(
            response.protocol,
            "PrihK96nBAFSxVL1GLJTVhu9YnzkMFiBeuJRPA8NwuZVZCE1L6i".to_string()
        );
        assert_eq!(response.chain_id, "NetXdQprcVkpaWU".to_string());
        assert_eq!(
            response.hash,
            "BLockGenesisGenesisGenesisGenesisGenesisf79b5d1CoW2".to_string()
        );
        assert_eq!(
            response.header.context,
            "CoV8SQumiVU9saiu3FVNeDNewJaJH8yWdsGF3WLdsRr2P9S7MzCj".to_string()
        );

        let block_metadata = response.metadata.expect("Block has metadata");
        assert_eq!(
            block_metadata.protocol,
            "PrihK96nBAFSxVL1GLJTVhu9YnzkMFiBeuJRPA8NwuZVZCE1L6i".to_string()
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

        let block_id = BlockID::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_1.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

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

        let block_id = BlockID::Level(2490368);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_ithaca.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

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

        let block_id = BlockID::Level(2504461);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string(), &block_id));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_jakarta.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

        client
            .get_block()
            .block_id(&block_id)
            .metadata(super::MetadataArg::Always)
            .send()
            .await?;

        Ok(())
    }
}
