use crate::models::checkpoint::Checkpoint;
use crate::client::TezosRPCContext;
use crate::error::Error;

fn path(chain_alias: String) -> String {
    format!("{}{}", super::path(chain_alias), "/checkpoint")
}

/// Get the current checkpoint for this chain.
///
/// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
pub async fn get(ctx: &TezosRPCContext) -> Result<Checkpoint, Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.get(path.as_str()).await
}
