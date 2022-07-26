use tezos_core::types::encoded::{Address, Encoded};

use crate::{client::TezosRpcChainId, http::Http};

pub mod balance;
pub mod counter;
pub mod delegate;
pub mod entrypoints;
pub mod manager_key;
pub mod script;

use {
    crate::client::TezosRpcContext, crate::error::Error, crate::models::contract::ContractInfo,
    crate::protocol_rpc::block::BlockId,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, contract: S) -> String {
    format!(
        "{}/contracts/{}",
        super::path(chain_id, block_id),
        contract.as_ref()
    )
}

/// A builder to construct the properties of a request to access the counter of a contract.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    contract: &'a Address,
    normalize_types: Option<bool>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, contract: &'a Address) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            contract,
            normalize_types: None,
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

    /// Whether types should be normalized (annotations removed, combs flattened) or kept as they appeared in the original script.
    pub fn normalize_types(mut self, normalize_types: bool) -> Self {
        self.normalize_types = Some(normalize_types);

        self
    }

    pub async fn send(&self) -> Result<ContractInfo, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(normalize_types) = self.normalize_types {
            // Add `normalize_types` query parameter
            query.push(("normalize_types", normalize_types.to_string()));
        }

        let path = self::path(self.chain_id.value(), self.block_id, self.contract.value());

        self.ctx
            .http_client()
            .get_with_query(path.as_str(), &Some(query))
            .await
    }
}

/// Access the complete status of a contract.
///
/// * `address` : A contract identifier encoded in b58check. e.g. `KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA`
///
/// Optional query arguments :
/// * `normalize_types` : Whether types should be normalized (annotations removed, combs flattened) or kept as they appeared in the original script.
///
/// [`GET ../<block_id>/context/contracts/<contract_id>?[normalize_types]`](https://tezos.gitlab.io/jakarta/rpc.html#get-block-id-context-contracts-contract-id)
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
        num_bigint::BigInt,
    };

    #[tokio::test]
    async fn test_get_contract() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockId::Level(1);
        let contract_address: Address = "KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA".try_into().unwrap();
        let normalize_types = true;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(
                    TezosRpcChainId::Main.value(),
                    &block_id,
                    contract_address.value(),
                ))
                .query_param("normalize_types", normalize_types.to_string());
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("contract/__TEST_DATA__/contract.json"));
        });

        let client = TezosRpc::new(rpc_url);
        let contract = client
            .get_contract(&contract_address)
            .normalize_types(normalize_types)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(contract.counter, None);
        assert_eq!(contract.delegate, None);
        assert_eq!(contract.balance, BigInt::from(0u8));

        let contract_script = contract.script.expect("Script exists");
        assert!(contract_script.storage.is_primitive_application());

        Ok(())
    }
}
