use {
    tezos_core::types::encoded::ChainID,
    crate::client::TezosRPCContext,
    crate::error::Error,
};

fn path(chain_id: String) -> String {
    format!("{}{}", super::path(chain_id),"/chain_id")
}

/// Get the chain unique identifier.
///
/// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
pub async fn get(ctx: &TezosRPCContext) -> Result<ChainID, Error> {
    let path = self::path(ctx.chain_id.to_string());

    ctx.http_client.get(path.as_str()).await
}

#[cfg(test)]
mod tests {
    use {
        httpmock::prelude::*,
        tezos_core::types::encoded::Encoded,
        crate::client::TezosRPC,
        crate::error::Error,
        crate::shell_rpc::ShellRPC,
    };

    #[tokio::test]
    async fn test_get_chain_id() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let chain_id_string = "NetXdQprcVkpaWU";

        server.mock(|when, then| {
            when.method(GET).path(super::path("main".to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(chain_id_string);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let chain_id = client.get_chain_id().await?;
        assert_eq!(chain_id_string, chain_id.base58());

        Ok(())
    }
}
