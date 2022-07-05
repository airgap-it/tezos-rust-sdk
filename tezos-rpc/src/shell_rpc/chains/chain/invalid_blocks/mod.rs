pub mod block;

use {
    crate::client::TezosRPCContext, crate::error::Error, crate::models::invalid_block::InvalidBlock,
};

fn path(chain_id: &String) -> String {
    format!("{}{}", super::path(chain_id), "/invalid_blocks")
}

/// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
///
/// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
pub async fn get(ctx: &TezosRPCContext) -> Result<Vec<InvalidBlock>, Error> {
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
    async fn test_get_invalid_blocks() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!(
            [
                {
                    "block": "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2",
                    "level": 2424833 as u64,
                    "errors": [
                        {
                            "kind": "permanent",
                            "id": "proto.alpha.Failed_to_get_script",
                            "contract": "KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW",
                        }
                    ]
                }
            ]
        );

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let response = client.get_invalid_blocks().await?;

        assert_eq!(response.len(), 1, "Expects a single invalid block.");

        let invalid_block = &response[0];
        assert_eq!(
            invalid_block.block.into_string(),
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
        );
        assert_eq!(invalid_block.level, 2424833);
        assert_eq!(invalid_block.errors.len(), 1, "Expects a single error.");
        assert_eq!(invalid_block.errors[0].kind, "permanent");
        assert_eq!(
            invalid_block.errors[0].id,
            "proto.alpha.Failed_to_get_script"
        );
        assert_eq!(
            invalid_block.errors[0].contract,
            Some("KT1XRPEPXbZK25r3Htzp2o1x7xdMMmfocKNW".to_string())
        );

        Ok(())
    }
}
