pub mod context;

use {
    crate::{
        client::TezosRPCContext,
        constants::{BLOCK_GENESIS_ALIAS, BLOCK_HEAD_ALIAS},
        error::Error,
        models::block::Block,
    },
    derive_more::Display,
    serde::{Deserialize, Serialize},
};

fn path(chain_id: String, block_id: String) -> String {
    format!("/chains/{}/blocks/{}", chain_id, block_id)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockID {
    Head,
    Genesis,
    Hash(String),
    Level(i32),
}

impl Default for BlockID {
    fn default() -> Self {
        BlockID::Head
    }
}

impl Default for &BlockID {
    fn default() -> Self {
        &BlockID::Head
    }
}

impl BlockID {
    fn value(&self) -> String {
        match self {
            Self::Head => BLOCK_HEAD_ALIAS.into(),
            Self::Genesis => BLOCK_GENESIS_ALIAS.into(),
            Self::Hash(hash) => hash.into(),
            Self::Level(level) => {
                if level.is_negative() {
                    return format!("head~{}", level.abs());
                }
                format!("{}", level)
            }
        }
    }
}

/// Specifies whether or not if the operations metadata should be returned. By default, the metadata will be returned depending on the node's metadata size limit policy.
///
/// To get the metadata, even if it is needed to recompute them, use [MetadataRPCArg::Always].
///
/// To avoid getting the metadata, use [MetadataRPCArg::Never].
#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetadataRPCArg {
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
pub async fn get(
    ctx: &TezosRPCContext,
    block_id: Option<&BlockID>,
    metadata: MetadataRPCArg,
) -> Result<Block, Error> {
    let path = self::path(
        ctx.chain_id.to_string(),
        block_id.unwrap_or_default().value(),
    );

    let mut query: Vec<(&str, String)> = vec![];

    // Add `metadata` query parameter
    query.push(("metadata", metadata.to_string()));

    ctx.http_client
        .get_with_query(path.as_str(), &Some(query))
        .await
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            client::TezosRPC,
            constants::{BLOCK_GENESIS_ALIAS, DEFAULT_CHAIN_ALIAS},
            error::Error,
            models::block::TestChainStatusName,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_genesis_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                DEFAULT_CHAIN_ALIAS.to_string(),
                BLOCK_GENESIS_ALIAS.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_genesis.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

        let block_id = BlockID::Genesis;
        let response = client
            .get_block(Some(&block_id), super::MetadataRPCArg::Always)
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

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(DEFAULT_CHAIN_ALIAS.to_string(), "1".into()));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_1.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

        client
            .get_block(Some(&BlockID::Level(1)), super::MetadataRPCArg::Always)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_ithaca_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                DEFAULT_CHAIN_ALIAS.to_string(),
                "2490368".into(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_ithaca.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

        client
            .get_block(
                Some(&BlockID::Level(2490368)),
                super::MetadataRPCArg::Always,
            )
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_jakarta_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                DEFAULT_CHAIN_ALIAS.to_string(),
                "2504461".into(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/block_jakarta.json"));
        });
        let client = TezosRPC::new(rpc_url.as_str());

        client
            .get_block(
                Some(&BlockID::Level(2504461)),
                super::MetadataRPCArg::Always,
            )
            .await?;

        Ok(())
    }
}
