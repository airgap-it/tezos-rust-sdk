use crate::{client::TezosRpcChainId, http::Http};

pub mod entrypoint;

use {
    crate::client::TezosRpcContext, crate::error::Error,
    crate::models::contract::ContractEntrypoints, crate::protocol_rpc::block::BlockID,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID, contract: S) -> String {
    format!("{}/entrypoints", super::path(chain_id, block_id, contract))
}

/// A builder to construct the properties of a request to obtain the list of entrypoints of a contract.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockID,
    contract: &'a str,
    normalize_types: Option<bool>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, contract: &'a str) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockID::Head,
            contract: contract,
            normalize_types: None,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a TezosRpcChainId) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockID) -> &mut Self {
        self.block_id = block_id;

        self
    }

    /// Whether the types should be normalized or not.
    ///
    /// For this to work, an `unparsing_mode` also needs to be provided.
    pub fn normalize_types(&mut self, normalize_types: bool) -> &mut Self {
        self.normalize_types = Some(normalize_types);

        self
    }

    pub async fn send(&self) -> Result<ContractEntrypoints, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(normalize_types) = self.normalize_types {
            // Add `normalize_types` query parameter
            query.push(("normalize_types", normalize_types.to_string()));
        }

        let path = self::path(self.chain_id.value(), self.block_id, self.contract);

        self.ctx
            .http_client()
            .get_with_query(path.as_str(), &Some(query))
            .await
    }
}

/// Return the list of entrypoints of a contract.
///
/// Optional query arguments:
///
/// * `normalize_types` : Whether types should be normalized (annotations removed, combs flattened) or kept as they appeared in the original script.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/entrypoints?[normalize_types]`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-entrypoints)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    address: &'a str,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, address)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::client::TezosRpcChainId;

    use {
        crate::{client::TezosRpc, error::Error, protocol_rpc::block::BlockID},
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_contract_entrypoints() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address = "KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA";
        let block_id = BlockID::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(
                    TezosRpcChainId::Main.value(),
                    &block_id,
                    &contract_address.to_string(),
                ))
                .query_param("normalize_types", "true");
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("entrypoints/__TEST_DATA__/entrypoints.json"));
        });

        let client = TezosRpc::new(rpc_url);
        let entrypoints = client
            .get_contract_entrypoints(&contract_address.to_string())
            .normalize_types(true)
            .block_id(&block_id)
            .send()
            .await?;

        assert!(entrypoints
            .entrypoints
            .get("mint")
            .unwrap()
            .is_micheline_primitive_application());
        assert_eq!(entrypoints.entrypoints.keys().len(), 23);

        Ok(())
    }
}
