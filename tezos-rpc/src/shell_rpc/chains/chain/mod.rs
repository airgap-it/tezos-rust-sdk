pub mod blocks;
pub mod chain_id;
pub mod invalid_blocks;
pub mod is_bootstrapped;
pub mod levels;

use {crate::client::TezosRPCContext, crate::error::Error, serde::Serialize};

fn path(chain_id: &String) -> String {
    format!("/chains/{}", chain_id)
}

/// `PatchChainPayload` used in request [`PATCH /chains/<chain_id>`](patch)
#[derive(Serialize)]
pub struct PatchChainPayload {
    /// A chain identifier. This is either a chain hash in Ba&se58Check notation or a one the predefined aliases: 'main', 'test'.
    bootstrapped: bool,
}

/// Forcefully set the bootstrapped flag of the node.
///
/// [`PATCH /chains/<chain_id>`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
pub async fn patch(ctx: &TezosRPCContext, body: &PatchChainPayload) -> Result<(), Error> {
    let path = self::path(&ctx.chain_id);

    ctx.http_client
        .patch::<_, serde_json::Value>(path.as_str(), &Some(body))
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_CHAIN_ALIAS;

    use {crate::client::TezosRPC, crate::error::Error, httpmock::MockServer};

    #[tokio::test]
    async fn test_patch_chain() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({}));
        });

        let client = TezosRPC::new(rpc_url.as_str());

        let req = super::PatchChainPayload {
            bootstrapped: false,
        };

        super::patch(&client.context, &req).await
    }
}
