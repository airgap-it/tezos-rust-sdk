use tezos_core::types::encoded::{Address, Encoded};

use crate::{client::TezosRpcChainId, http::Http};

use {crate::client::TezosRpcContext, crate::error::Error, crate::protocol_rpc::block::BlockId};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, contract: S) -> String {
    format!("{}/manager_key", super::path(chain_id, block_id, contract))
}

/// A builder to construct the properties of a request to access the manager public key of a contract.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    contract: &'a Address,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, contract: &'a Address) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            contract,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(mut self, block_id: &'a BlockId) -> Self {
        self.block_id = block_id;

        self
    }

    pub async fn send(&self) -> Result<Option<String>, Error> {
        let path = self::path(self.chain_id.value(), self.block_id, self.contract.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Access the manager public key of a contract.
///
/// [`GET /chains/<chain_id>/blocks/<block_id>/context/contracts/<contract_id>/manager_key`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-manager-key)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    address: &'a Address,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, address)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use tezos_core::types::encoded::{Address, Encoded};

    use crate::client::TezosRpcChainId;

    use {
        crate::{client::TezosRpc, error::Error, protocol_rpc::block::BlockId},
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_manager_key() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address: Address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".try_into().unwrap();
        let expected_manager = "edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg";
        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                TezosRpcChainId::Main.value(),
                &block_id,
                contract_address.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(expected_manager);
        });

        let client = TezosRpc::new(rpc_url);
        let manager_key = client
            .get_contract_manager_key(&contract_address)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(manager_key, Some(expected_manager.into()));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_null_manager_key() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address: Address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".try_into().unwrap();
        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                TezosRpcChainId::Main.value(),
                &block_id,
                contract_address.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body("null");
        });

        let client = TezosRpc::new(rpc_url);
        let counter = client
            .get_contract_manager_key(&contract_address)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(counter, None);

        Ok(())
    }
}
