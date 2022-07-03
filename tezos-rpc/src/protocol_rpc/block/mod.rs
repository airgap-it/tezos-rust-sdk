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

// impl Copy for BlockID {
// }

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
