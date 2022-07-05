use {crate::client::TezosRPCContext, crate::error::Error};

fn path(chain_id: &String) -> String {
    format!("{}/chain_id", super::path(chain_id))
}

/// A builder to construct the properties of a request to get the chain unique identifier.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a String,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a String) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    pub async fn send(self) -> Result<String, Error> {
        let path = self::path(self.chain_id);

        self.ctx.http_client.get(path.as_str()).await
    }
}

/// Get the chain unique identifier.
///
/// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
pub fn get(ctx: &TezosRPCContext) -> RPCRequestBuilder {
    RPCRequestBuilder::new(ctx)
}

#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_CHAIN_ALIAS;

    use {crate::client::TezosRPC, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_chain_id() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let chain_id_string = "NetXdQprcVkpaWU";

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(&DEFAULT_CHAIN_ALIAS.to_string()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(chain_id_string);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let chain_id = client.get_chain_id().send().await?;
        assert_eq!(chain_id_string, chain_id);

        Ok(())
    }
}
