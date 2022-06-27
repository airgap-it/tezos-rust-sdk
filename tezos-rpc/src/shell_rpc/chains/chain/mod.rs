pub mod chain_id;
pub mod invalid_blocks;
pub mod levels;
pub mod blocks;

use serde::Serialize;
use crate::client::TezosRPCContext;
use crate::error::Error;

fn path(chain_alias: String) -> String {
    format!("/chains/{}", chain_alias)
}

/// `PatchChainRequest` is used as body in request:
///
/// [`PATCH /chains/<chain_id>`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
#[derive(Serialize)]
pub struct PatchChainRequest {
    /// A chain identifier. This is either a chain hash in Base58Check notation or a one the predefined aliases: 'main', 'test'.
    bootstrapped: bool
}

/// Forcefully set the bootstrapped flag of the node.
///
/// [`PATCH /chains/<chain_id>`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
pub async fn patch(ctx: &TezosRPCContext, body: PatchChainRequest) -> Result<(), Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.patch(path.as_str(), &Some(body)).await?;

    Ok(())
}
