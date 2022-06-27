pub mod chain_id;
pub mod invalid_blocks;
pub mod levels;
pub mod blocks;
pub mod is_bootstrapped;

use {
    serde::Serialize,
    crate::client::TezosRPCContext,
    crate::error::Error
};

fn path(chain_id: String) -> String {
    format!("/chains/{}", chain_id)
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
pub async fn patch(ctx: &TezosRPCContext, body: &PatchChainRequest) -> Result<(), Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.patch::<_, serde_json::Value>(path.as_str(), &Some(body)).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        httpmock::prelude::*,
        crate::client::TezosRPC,
        crate::error::Error,
        crate::shell_rpc::ShellRPC,
        super::PatchChainRequest
    };

    #[tokio::test]
    async fn test_patch_chain() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH).path(super::path("main".to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({}));
        });

        let client = TezosRPC::new(rpc_url.as_str());

        let req = PatchChainRequest {
            bootstrapped: false
        };

        client.patch_chain(&req).await
    }
}
