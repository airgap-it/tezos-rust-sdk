use crate::http::Http;

use {
    crate::client::TezosRpcContext, crate::error::Error, crate::models::contract::ContractScript,
    crate::models::contract::UnparsingMode, crate::protocol_rpc::block::BlockID, serde::Serialize,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID, contract: S) -> String {
    format!("{}/script", super::path(chain_id, block_id, contract))
}

#[derive(Serialize)]
struct NormalizedPayload {
    unparsing_mode: UnparsingMode,
    normalize_types: Option<bool>,
}

/// A builder to construct the properties of a request to access the code and data of the contract.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a str,
    block_id: &'a BlockID,
    contract: &'a str,
    unparsing_mode: Option<UnparsingMode>,
    normalize_types: Option<bool>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, contract: &'a str) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockID::Head,
            contract: contract,
            unparsing_mode: None,
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

    /// Normalize the script using an unparsing mode.
    ///
    /// Parsing modes:
    ///
    /// * [UnparsingMode::Readable]
    /// * [UnparsingMode::Optimized]
    /// * [UnparsingMode::Optimized_legacy]
    pub fn unparsing_mode(&mut self, parsing_mode: UnparsingMode) -> &mut Self {
        self.unparsing_mode = Some(parsing_mode);

        self
    }

    /// Whether the types should be normalized or not.
    ///
    /// For this to work, an `unparsing_mode` also needs to be provided.
    pub fn normalize_types(&mut self, normalize_types: bool) -> &mut Self {
        self.normalize_types = Some(normalize_types);

        self
    }

    pub async fn send(&self) -> Result<ContractScript, Error> {
        if self.unparsing_mode.is_none() {
            let path = self::path(self.chain_id, self.block_id, self.contract);

            self.ctx.http_client().get(path.as_str()).await
        } else {
            let path = format!(
                "{}/normalized",
                self::path(self.chain_id, self.block_id, self.contract)
            );

            let param = NormalizedPayload {
                unparsing_mode: self.unparsing_mode.unwrap(),
                normalize_types: self.normalize_types,
            };

            self.ctx
                .http_client()
                .post::<_, _, ()>(path.as_str(), &param, &None)
                .await
        }
    }
}

/// Access the code and data of the contract.
///
/// [`GET /chains/<chain_id>/blocks/<block_id>/context/contracts/<contract_id>/script`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-script)
///
/// If `unparsing_mode` is provided, the request below will be used.
///
/// [`POST /chains/<chain_id>/blocks/<block_id>/context/contracts/<contract_id>/script/normalized`](https://tezos.gitlab.io/active/rpc.html#post-block-id-context-contracts-contract-id-script-normalized)
pub fn get_or_post<'a, HttpClient: Http>(
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
            models::contract::UnparsingMode, protocol_rpc::block::BlockID,
        },
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_script() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL";
        let block_id = BlockID::Level(1);

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                &DEFAULT_CHAIN_ALIAS.to_string(),
                &block_id,
                &contract_address.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/optimized_contract_script.json"));
        });

        let client = TezosRpc::new(rpc_url);
        let script = client
            .get_contract_script(&contract_address.to_string())
            .block_id(&block_id)
            .send()
            .await?;

        assert!(script.code.is_micheline_sequence());
        assert!(script.storage.is_micheline_sequence());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_normalized_script() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL";
        let block_id = BlockID::Level(1);
        let unparsing_mode = UnparsingMode::Readable;
        let normalize_types = true;

        let body = serde_json::to_string(&super::NormalizedPayload {
            unparsing_mode: unparsing_mode,
            normalize_types: Some(normalize_types),
        })?;

        server.mock(|when, then| {
            when.method(POST)
                .path(format!(
                    "{}/normalized",
                    super::path(
                        &DEFAULT_CHAIN_ALIAS.to_string(),
                        &block_id,
                        &contract_address.to_string(),
                    )
                ))
                .body(body);
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/optimized_contract_script.json"));
        });

        let client = TezosRpc::new(rpc_url);
        let script = client
            .get_contract_script(&contract_address.to_string())
            .unparsing_mode(UnparsingMode::Readable)
            .normalize_types(true)
            .block_id(&block_id)
            .send()
            .await?;

        assert!(script.code.is_micheline_sequence());
        assert!(script.storage.is_micheline_sequence());

        Ok(())
    }
}
