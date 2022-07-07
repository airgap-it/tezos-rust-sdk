use tezos_michelson::micheline::Micheline;

use {crate::client::TezosRPCContext, crate::error::Error, crate::protocol_rpc::block::BlockID};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID, contract: S, entrypoint: S) -> String {
    format!(
        "{}/{}",
        super::path(chain_id, block_id, contract),
        entrypoint.as_ref()
    )
}

/// A builder to construct the properties of a request to obtain the type of a given entrypoint.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a str,
    block_id: &'a BlockID,
    contract: &'a str,
    entrypoint: &'a str,
    normalize_types: Option<bool>,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext, contract: &'a str, entrypoint: &'a str) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
            block_id: &BlockID::Head,
            contract,
            entrypoint,
            normalize_types: None,
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

    /// Whether the types should be normalized or not.
    ///
    /// For this to work, an `unparsing_mode` also needs to be provided.
    pub fn normalize_types(&mut self, normalize_types: bool) -> &mut Self {
        self.normalize_types = Some(normalize_types);

        self
    }

    pub async fn send(self) -> Result<Micheline, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(normalize_types) = self.normalize_types {
            // Add `normalize_types` query parameter
            query.push(("normalize_types", normalize_types.to_string()));
        }

        let path = self::path(self.chain_id, self.block_id, self.contract, self.entrypoint);

        self.ctx
            .http_client
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
pub fn get<'a>(
    ctx: &'a TezosRPCContext,
    address: &'a str,
    entrypoint: &'a str,
) -> RPCRequestBuilder<'a> {
    RPCRequestBuilder::new(ctx, address, entrypoint)
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            client::TezosRPC, constants::DEFAULT_CHAIN_ALIAS, error::Error,
            protocol_rpc::block::BlockID,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_contract_entrypoint() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address = "KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA";
        let entrypoint = "settle_with_vault";
        let block_id = BlockID::Level(1);

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(
                    &DEFAULT_CHAIN_ALIAS.to_string(),
                    &block_id,
                    &contract_address.to_string(),
                    &entrypoint.to_string(),
                ))
                .query_param("normalize_types", "true");
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/entrypoint.json"));
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let entrypoints = client
            .get_contract_entrypoint(&contract_address.to_string(), &entrypoint.to_string())
            .normalize_types(true)
            .block_id(&block_id)
            .send()
            .await?;

        assert!(entrypoints.is_micheline_primitive_application());

        Ok(())
    }
}