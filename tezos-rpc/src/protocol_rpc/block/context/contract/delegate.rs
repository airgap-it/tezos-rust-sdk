use crate::http::Http;

use {crate::client::TezosRpcContext, crate::error::Error, crate::protocol_rpc::block::BlockID};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID, contract: S) -> String {
    format!("{}/delegate", super::path(chain_id, block_id, contract))
}

/// A builder to construct the properties of a request to access the delegate of a contract.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a str,
    block_id: &'a BlockID,
    contract: &'a str,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, contract: &'a str) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockID::Head,
            contract: contract,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a str) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockID) -> &mut Self {
        self.block_id = block_id;

        self
    }

    pub async fn send(&self) -> Result<Option<String>, Error> {
        let path = self::path(self.chain_id, self.block_id, self.contract);

        match self.ctx.http_client().get(path.as_str()).await {
            Ok(delegate) => Ok(Some(delegate)),
            Err(_) => Ok(None),
        }
    }
}

/// Access the delegate of a contract, if any.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/delegate`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-delegate)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    address: &'a str,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, address)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use {
        crate::{
            client::TezosRpc, constants::DEFAULT_CHAIN_ALIAS, error::Error,
            protocol_rpc::block::BlockID,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_delegate() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL";
        let expected_delegate = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL";
        let block_id = BlockID::Level(1);

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                &DEFAULT_CHAIN_ALIAS.to_string(),
                &block_id,
                &contract_address.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(expected_delegate);
        });

        let client = TezosRpc::new(rpc_url);
        let delegate = client
            .get_contract_delegate(&contract_address.to_string())
            .block_id(&block_id)
            .send()
            .await?;
        assert_eq!(delegate, Some(expected_delegate.to_string()));

        Ok(())
    }
}
