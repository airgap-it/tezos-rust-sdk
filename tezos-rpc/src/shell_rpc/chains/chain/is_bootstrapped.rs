use {
    crate::client::TezosRPCContext, crate::error::Error,
    crate::models::bootstrapped_status::BootstrappedStatus,
};

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/is_bootstrapped", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the bootstrap status of a chain.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a str,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a str) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(self) -> Result<BootstrappedStatus, Error> {
        let path = self::path(self.chain_id);

        self.ctx.http_client.get(path.as_str()).await
    }
}

/// Get the bootstrap status of a chain.
///
/// [`GET /chains/<chain_id>/is_bootstrapped`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-is-bootstrapped)
pub fn get(ctx: &TezosRPCContext) -> RPCRequestBuilder {
    RPCRequestBuilder::new(ctx)
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

        let response = client.is_bootstrapped().send().await?;

        assert_eq!(response.bootstrapped, false);
        assert_eq!(response.sync_state, ChainStatus::Stuck);

        Ok(())
    }
}
