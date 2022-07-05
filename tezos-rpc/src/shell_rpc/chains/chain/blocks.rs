use crate::client::TezosRPCContext;
use crate::error::Error;
use serde::Serialize;
use tezos_core::types::encoded::BlockHash;

fn path(chain_id: &String) -> String {
    format!("{}/blocks", super::path(chain_id))
}

/// `GetBlocksQuery` query parameters for request:
///
/// [`GET /chains/<chain_id>/blocks?[length=<uint>]&(head=<block_hash>)&[min_date=<date>]`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
#[derive(Serialize)]
pub struct GetBlocksQuery {
    /// The requested number of predecessors to return.
    pub length: Option<u32>,
    /// Requests blocks starting with the current head if `None` is provided.
    pub head: Option<BlockHash>,
    /// A date in seconds from epoch.
    /// When `min_date` is provided, blocks with a timestamp before `min_date` are filtered out.
    pub min_date: Option<u64>,
}

/// Lists block hashes from `<chain>`, up to the last checkpoint, sorted with
/// decreasing fitness. Without arguments it returns the head of the chain.
///
/// Optional arguments [GetBlocksQuery] allow to return the list of predecessors of a given block or of a set of blocks.
///
/// [`GET /chains/<chain_id>/blocks`](https://tezos.gitlab.io/shell/rpc.html#get_chains__chain_id__blocks)
pub async fn get(
    ctx: &TezosRPCContext,
    query: &GetBlocksQuery,
) -> Result<Vec<Vec<BlockHash>>, Error> {
    let path = self::path(&ctx.chain_id);

    ctx.http_client.get_with_query(path.as_str(), query).await
}

#[cfg(test)]
mod tests {

    use {
        crate::client::TezosRPC,
        crate::constants::DEFAULT_CHAIN_ALIAS,
        crate::error::Error,
        crate::shell_rpc::chains::chain::blocks::GetBlocksQuery,
        httpmock::prelude::*,
        tezos_core::types::encoded::{BlockHash, Encoded},
    };

    #[tokio::test]
    async fn test_is_bootstrapped() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response =
            serde_json::json!([["BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8"]]);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()))
                .query_param("length", "1")
                .query_param(
                    "head",
                    "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8",
                )
                .query_param("min_date", "1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRPC::new(rpc_url.as_str());

        let req_query = &GetBlocksQuery {
            length: Some(1),
            head: Some(BlockHash::new(
                "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8".to_string(),
            )?),
            min_date: Some(1),
        };
        let response = client.get_blocks(req_query).await?;

        assert_eq!(
            response[0][0].into_string(),
            "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8"
        );

        Ok(())
    }
}
