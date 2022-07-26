use tezos_core::types::encoded::{Address, Encoded};

use crate::{client::TezosRpcChainId, http::Http};

use {
    crate::client::TezosRpcContext, crate::error::Error, crate::protocol_rpc::block::BlockId,
    num_bigint::BigUint,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, contract: S) -> String {
    format!("{}/counter", super::path(chain_id, block_id, contract))
}

/// A builder to construct the properties of a request to access the counter of a contract.
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
            contract: contract,
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

    pub async fn send(&self) -> Result<BigUint, Error> {
        let path = self::path(self.chain_id.value(), self.block_id, self.contract.value());

        let balance: String = self.ctx.http_client().get(path.as_str()).await?;

        Ok(balance.parse::<BigUint>()?)
    }
}

/// Access the counter of a contract.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/counter`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-counter)
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
        num_bigint::BigUint,
    };

    #[tokio::test]
    async fn test_get_counter() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address: Address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".try_into().unwrap();
        let expected_counter = BigUint::from(9999999999999999999 as u64);
        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                TezosRpcChainId::Main.value(),
                &block_id,
                contract_address.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(format!("{}", expected_counter));
        });

        let client = TezosRpc::new(rpc_url);
        let counter = client
            .get_contract_counter(&contract_address)
            .block_id(&block_id)
            .send()
            .await?;
        assert_eq!(counter, expected_counter);

        Ok(())
    }
}
