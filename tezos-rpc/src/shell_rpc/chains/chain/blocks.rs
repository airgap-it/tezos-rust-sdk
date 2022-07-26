use tezos_core::types::encoded::{BlockHash, Encoded};

use crate::client::{TezosRpcChainId, TezosRpcContext};
use crate::error::Error;
use crate::http::Http;

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/blocks", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the chain unique identifier.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    /// The requested number of predecessors to return.
    length: Option<u32>,
    /// Requests blocks starting with the current head if `None` is provided.
    head: Option<&'a BlockHash>,
    /// A date in seconds from epoch.
    /// When `min_date` is provided, blocks with a timestamp before `min_date` are filtered out.
    min_date: Option<u64>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            length: None,
            head: None,
            min_date: None,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    /// Set the requested number of predecessors to return.
    pub fn length(mut self, length: &u32) -> Self {
        self.length = Some(length.clone());

        self
    }

    /// Request blocks starting from a given block.
    pub fn head(mut self, head: &'a BlockHash) -> Self {
        self.head = Some(head);

        self
    }

    /// A date in seconds from epoch.
    /// When `min_date` is provided, blocks with a timestamp before `min_date` are filtered out.
    pub fn min_date(mut self, min_date: &u64) -> Self {
        self.min_date = Some(min_date.clone());

        self
    }

    pub async fn send(&self) -> Result<Vec<Vec<String>>, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(length) = self.length {
            // Add `length` query parameter
            query.push(("length", length.to_string()));
        }
        if let Some(head) = self.head {
            // Add `head` query parameter
            query.push(("head", head.value().into()));
        }
        if let Some(min_date) = self.min_date {
            // Add `min_date` query parameter
            query.push(("min_date", min_date.to_string()));
        }

        self.ctx
            .http_client()
            .get_with_query(self::path(self.chain_id.value()).as_str(), &Some(query))
            .await
    }
}

/// Lists block hashes from `<chain>`, up to the last checkpoint, sorted with
/// decreasing fitness. Without arguments it returns the head of the chain.
///
/// [`GET /chains/<chain_id>/blocks`](https://tezos.gitlab.io/shell/rpc.html#get_chains__chain_id__blocks)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_blocks() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response =
            serde_json::json!([["BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8"]]);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&TezosRpcChainId::Main.value()))
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

        let client = TezosRpc::new(rpc_url);

        let response = client
            .get_blocks()
            .length(&1)
            .head(
                &"BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8"
                    .try_into()
                    .unwrap(),
            )
            .min_date(&1)
            .send()
            .await?;

        assert_eq!(
            response[0][0],
            "BMaCWKEayxSBRFMLongZCjAnLREtFC5Shnqb6v8qdcLsDZvZPq8"
        );

        Ok(())
    }
}
