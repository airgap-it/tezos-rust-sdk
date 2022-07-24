use tezos_core::types::encoded::{Address, Encoded};
use tezos_michelson::micheline::Micheline;

use crate::{client::TezosRpcChainId, http::Http};

use {crate::client::TezosRpcContext, crate::error::Error, crate::protocol_rpc::block::BlockId};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, contract: S, entrypoint: S) -> String {
    format!(
        "{}/{}",
        super::path(chain_id, block_id, contract),
        entrypoint.as_ref()
    )
}

/// A builder to construct the properties of a request to obtain the type of a given entrypoint.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    contract: &'a Address,
    entrypoint: &'a str,
    normalize_types: Option<bool>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(
        ctx: &'a TezosRpcContext<HttpClient>,
        contract: &'a Address,
        entrypoint: &'a str,
    ) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            contract,
            entrypoint,
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

    /// Whether the types should be normalized or not.
    ///
    /// For this to work, an `unparsing_mode` also needs to be provided.
    pub fn normalize_types(mut self, normalize_types: bool) -> Self {
        self.normalize_types = Some(normalize_types);

        self
    }

    pub async fn send(&self) -> Result<Micheline, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(normalize_types) = self.normalize_types {
            // Add `normalize_types` query parameter
            query.push(("normalize_types", normalize_types.to_string()));
        }

        let path = self::path(
            self.chain_id.value(),
            self.block_id,
            self.contract.value(),
            self.entrypoint,
        );

        self.ctx
            .http_client()
            .get_with_query(path.as_str(), &Some(query))
            .await
    }
}

/// Return the type of a given entrypoint of a contract.
///
/// Optional query arguments:
///
/// * `normalize_types` : Whether types should be normalized (annotations removed, combs flattened) or kept as they appeared in the original script.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/entrypoints/<entrypoint>?[normalize_types]`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-entrypoints)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    address: &'a Address,
    entrypoint: &'a str,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, address, entrypoint)
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
    async fn test_get_contract_entrypoint() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address: Address = "KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA".try_into().unwrap();
        let entrypoint = "settle_with_vault";
        let block_id = BlockId::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(
                    TezosRpcChainId::Main.value(),
                    &block_id,
                    contract_address.value(),
                    entrypoint,
                ))
                .query_param("normalize_types", "true");
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/entrypoint.json"));
        });

        let client = TezosRpc::new(rpc_url);
        let entrypoints = client
            .get_contract_entrypoint(&contract_address, entrypoint)
            .normalize_types(true)
            .block_id(&block_id)
            .send()
            .await?;

        assert!(entrypoints.is_primitive_application());

        Ok(())
    }
}
