use serde::Serialize;
use tezos_core::types::encoded::{BlockHash};
use crate::client::TezosRPCContext;
use crate::error::Error;

fn path(chain_alias: String) -> String {
    format!("{}{}", super::path(chain_alias),"/blocks")
}

/// `GetBlocksQuery` query parameters for request:
///
/// [`GET /chains/<chain_id>/blocks?[length=<uint>]&(head=<block_hash>)*&[min_date=<date>]`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
#[derive(Serialize)]
pub struct GetBlocksQuery {
    /// The requested number of predecessors to return.
    pub length: Option<u32>,
    /// Requests blocks starting with the current head if `None` is provided.
    pub head: Option<BlockHash>,
    /// A date in seconds from epoch.
    /// When `min_date` is provided, blocks with a timestamp before `min_date` are filtered out.
    pub min_date: Option<u64>
}

/// Lists block hashes from `<chain>`, up to the last checkpoint, sorted with
/// decreasing fitness. Without arguments it returns the head of the chain.
///
/// Optional arguments [GetBlocksQuery] allow to return the list of predecessors of a given block or of a set of blocks.
///
/// [`GET /chains/<chain_id>/blocks`](https://tezos.gitlab.io/shell/rpc.html#get_chains__chain_id__blocks)
pub async fn get(ctx: &TezosRPCContext, query: &GetBlocksQuery) -> Result<Vec<Vec<BlockHash>>, Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.get_with_query(path.as_str(), query).await
}
