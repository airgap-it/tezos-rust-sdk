use tezos_core::types::encoded::{ChainID};
use crate::client::TezosRPCContext;
use crate::error::Error;

fn path(chain_alias: String) -> String {
    format!("{}{}", super::path(chain_alias),"/chain_id")
}

/// Get the chain unique identifier.
///
/// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
pub async fn get(ctx: &TezosRPCContext) -> Result<ChainID, Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.get(path.as_str()).await
}
