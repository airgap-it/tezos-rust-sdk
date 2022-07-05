use {crate::client::TezosRPCContext, crate::error::Error, crate::models::checkpoint::Checkpoint};

fn path(chain_id: &String) -> String {
    format!("{}{}", super::path(chain_id), "/checkpoint")
}

/// Get the current checkpoint for this chain.
///
/// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
pub async fn get(ctx: &TezosRPCContext) -> Result<Checkpoint, Error> {
    let path = self::path(&ctx.chain_id);

    ctx.http_client.get(path.as_str()).await
}

#[cfg(test)]
mod tests {
    use {
        crate::client::TezosRPC, crate::constants::DEFAULT_CHAIN_ALIAS, crate::error::Error,
        httpmock::prelude::*, tezos_core::types::encoded::Encoded,
    };

    #[tokio::test]
    async fn test_get_checkpoint() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!({
            "block_hash": "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2",
            "level": 2424833 as u64
        });

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let response = client.get_checkpoint().await?;

        assert_eq!(
            response.block_hash.into_string(),
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
        );
        assert_eq!(response.level, 2424833);

        Ok(())
    }
}
