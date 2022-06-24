use crate::{models::checkpoint::Checkpoint, client::RpcContext, error::Error};

fn path(chain_alias: String) -> String {
    format!("/chains/{}/levels", chain_alias)
}

/// Get the current checkpoint for this chain.
///
/// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
pub async fn checkpoint(ctx: &RpcContext) -> Result<Checkpoint, Error> {
    let path = format!("{}/checkpoint", self::path(ctx.chain_alias.to_string()));

    let checkpoint = ctx.http_client.get(path.as_str()).await?;

    Ok(checkpoint)
}
