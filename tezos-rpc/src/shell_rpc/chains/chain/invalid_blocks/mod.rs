use crate::client::TezosRPCContext;
use crate::error::Error;
use crate::models::invalid_block::InvalidBlock;

fn path(chain_alias: String) -> String {
    format!("{}{}", super::path(chain_alias),"/invalid_blocks")
}

/// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
///
/// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
pub async fn get(ctx: &TezosRPCContext) -> Result<Vec<InvalidBlock>, Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.get(path.as_str()).await
}
