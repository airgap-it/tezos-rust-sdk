use crate::http::Http;

use {crate::client::TezosRPCContext, crate::error::Error, crate::models::checkpoint::Checkpoint};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/savepoint", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the current savepoint for this chain.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRPCContext<HttpClient>,
    chain_id: &'a str,
}

impl<'a, HttpClient: Http> RPCRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRPCContext<HttpClient>) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a str) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(&self) -> Result<Checkpoint, Error> {
        let path = self::path(self.chain_id);

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Get the current savepoint for this chain.
///
/// [`GET /chains/<chain_id>/levels/savepoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-savepoint)
pub fn get<HttpClient: Http>(ctx: &TezosRPCContext<HttpClient>) -> RPCRequestBuilder<HttpClient> {
    RPCRequestBuilder::new(ctx)
}

#[cfg(test)]
mod tests {
    use {
        crate::client::TezosRPC, crate::constants::DEFAULT_CHAIN_ALIAS, crate::error::Error,
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_savepoint() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let valid_response = serde_json::json!(
            {
                "block_hash": "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2",
                "level": 2424833 as u64
            }
        );

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(valid_response);
        });

        let client = TezosRPC::new(rpc_url);
        let response = client.get_savepoint().send().await?;

        assert_eq!(
            response.block_hash,
            "BLY6dM4iqKHxjAJb2P9dRVEroejqYx71qFddGVCk1wn9wzSs1S2"
        );
        assert_eq!(response.level, 2424833);

        Ok(())
    }
}
