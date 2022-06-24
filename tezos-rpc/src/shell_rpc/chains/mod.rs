pub mod levels;

use tezos_core::types::encoded::{ChainID, Encoded};
use crate::client::RpcContext;
use crate::error::Error;
use crate::models::invalid_block::InvalidBlock;

fn path(chain_alias: String) -> String {
    format!("/chains/{}", chain_alias)
}

/// Get the chain unique identifier.
///
/// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
pub async fn chain_id(ctx: &RpcContext) -> Result<ChainID, Error> {
    let path = format!("{}/chain_id", self::path(ctx.chain_alias.to_string()));

    let chain_id = ctx.http_client.get(path.as_str()).await?;

    Ok(ChainID::new(chain_id)?)
}

/// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
///
/// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
pub async fn invalid_blocks(ctx: &RpcContext) -> Result<Vec<InvalidBlock>, Error> {
    let path = format!("{}/invalid_blocks", self::path(ctx.chain_alias.to_string()));

    let invalid_blocks = ctx.http_client.get(path.as_str()).await?;

    Ok(invalid_blocks)
}
