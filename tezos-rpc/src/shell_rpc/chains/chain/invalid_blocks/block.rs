use {
    crate::client::TezosRPCContext,
    crate::error::Error,
    crate::models::invalid_block::InvalidBlock,
    tezos_core::types::encoded::{BlockHash, Encoded},
};

fn path(chain_id: String, block_hash: String) -> String {
    format!("{}/{}", super::path(chain_id), block_hash)
}

/// Get the errors that appeared during the block (in)validation.
///
/// [`GET /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks-block-hash)
pub async fn get(ctx: &TezosRPCContext, block_hash: &BlockHash) -> Result<InvalidBlock, Error> {
    let path = self::path(ctx.chain_id.to_string(), block_hash.into_string());

    ctx.http_client.get(path.as_str()).await
}

/// Get the errors that appeared during the block (in)validation.
///
/// [`GET /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks-block-hash)
pub async fn delete(ctx: &TezosRPCContext, block_hash: &BlockHash) -> Result<(), Error> {
    let path = self::path(ctx.chain_id.to_string(), block_hash.into_string());

    ctx.http_client
        .delete::<(), serde_json::Value>(path.as_str(), &None)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        crate::client::TezosRPC,
        crate::error::Error,
        crate::shell_rpc::ShellRPC,
        httpmock::prelude::*,
        tezos_core::types::encoded::{BlockHash, Encoded},
    };

    #[tokio::test]
    async fn test_get_invalid_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let invalid_block_hash = "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2";
        let valid_response = serde_json::json!(
            {
                "block": invalid_block_hash,
                "level": 2424833 as u64,
                "errors": [
                    {
                        "kind": "permanent",
                        "id": "proto.alpha.Failed_to_get_script",
                        "contract": "KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW",
                    }
                ]
            }
        );

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                "main".to_string(),
                invalid_block_hash.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let response = client
            .get_invalid_block(&BlockHash::new(invalid_block_hash.to_string())?)
            .await?;

        assert_eq!(
            response.block.into_string(),
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
        );
        assert_eq!(response.level, 2424833);
        assert_eq!(response.errors.len(), 1, "Expects a single error.");
        assert_eq!(response.errors[0].kind, "permanent");
        assert_eq!(response.errors[0].id, "proto.alpha.Failed_to_get_script");
        assert_eq!(
            response.errors[0].contract,
            Some("KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW".to_string())
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_invalid_block() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let invalid_block_hash = "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2";

        server.mock(|when, then| {
            when.method(DELETE).path(super::path(
                "main".to_string(),
                invalid_block_hash.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({}));
        });

        let client = TezosRPC::new(rpc_url.as_str());

        client
            .remove_invalid_block(&BlockHash::new(invalid_block_hash.to_string())?)
            .await
    }
}
