use {
    crate::client::TezosRPCContext, crate::error::Error,
    crate::models::bootstrapped_status::BootstrappedStatus,
};

fn path(chain_id: &String) -> String {
    format!("{}{}", super::path(chain_id), "/is_bootstrapped")
}

/// Get the bootstrap status of a chain.
///
/// [`GET /chains/<chain_id>/is_bootstrapped`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-is-bootstrapped)
pub async fn get(ctx: &TezosRPCContext) -> Result<BootstrappedStatus, Error> {
    let path = self::path(&ctx.chain_id);

    ctx.http_client.get(path.as_str()).await
}

#[cfg(test)]
mod tests {
    use {
        crate::client::TezosRPC, crate::constants::DEFAULT_CHAIN_ALIAS, crate::error::Error,
        crate::models::bootstrapped_status::ChainStatus, httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_is_bootstrapped() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!(
            {
                "bootstrapped": false,
                "sync_state": "stuck"
            }
        );

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });
        let client = TezosRPC::new(rpc_url.as_str());

        let response = client.is_bootstrapped().await?;

        assert_eq!(response.bootstrapped, false);
        assert_eq!(response.sync_state, ChainStatus::Stuck);

        Ok(())
    }
}
