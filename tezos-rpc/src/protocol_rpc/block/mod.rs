use {
    crate::client::TezosRPCContext,
    crate::error::Error,
    crate::models::block::Block,
    derive_more::Display,
    serde::{Deserialize, Serialize},
};

fn path(chain_id: String, block_id: String) -> String {
    format!("/chains/{}/blocks/{}", chain_id, block_id)
}

#[derive(Debug, Clone)]
pub enum BlockID {
    Head,
    Hash(String),
    Level(i32),
}

impl Default for BlockID {
    fn default() -> Self {
        BlockID::Head
    }
}

impl BlockID {
    fn value(&self) -> String {
        match self {
            Self::Head => "head".into(),
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
    block_id: &Option<BlockID>,
    metadata: MetadataRPCArg,
) -> Result<Block, Error> {
    let path = self::path(
        ctx.chain_id.to_string(),
        block_id.clone().unwrap_or_default().value(),
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
        crate::constants::{DEFAULT_CHAIN_ALIAS, BLOCK_HEAD_ALIAS},
        crate::models::block::Block,
        crate::client::TezosRPC, crate::error::Error,
        crate::protocol_rpc::ProtocolRPC,
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_json = include_str!("test_data/block_simple.json");
        let block: Block = serde_json::from_str(&block_json)?;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(DEFAULT_CHAIN_ALIAS.to_string(), BLOCK_HEAD_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .body(block_json);
        });
        let client = TezosRPC::new(rpc_url.as_str());

        let response = client.get_block(&None, super::MetadataRPCArg::Always).await?;

        assert!(response.protocol.starts_with("P"));
        assert!(response.chain_id.starts_with("Net"));
        assert!(response.hash.starts_with("B"));
        assert_eq!(response.header, block.header);

        let expected_block_metadata = block.metadata.expect("Block has metadata");
        let block_metadata = response.metadata.expect("Block has metadata");
        assert_eq!(block_metadata, expected_block_metadata);

        Ok(())
    }
}
