use tezos_core::types::encoded::{Encoded, ScriptExprHash};

use crate::{client::TezosRpcChainId, http::Http};

use {
    crate::client::TezosRpcContext, crate::error::Error, crate::models::contract::UnparsingMode,
    crate::protocol_rpc::block::BlockId, serde::Serialize, tezos_michelson::micheline::Micheline,
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, big_map_id: u32, key: S) -> String {
    format!(
        "{}/{}",
        super::path(chain_id, block_id, big_map_id),
        key.as_ref()
    )
}

#[derive(Serialize)]
struct NormalizedPayload {
    unparsing_mode: UnparsingMode,
}

/// A builder to construct the properties of a request to access the value associated with a key in a big map.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    big_map_id: u32,
    script_expr: &'a ScriptExprHash,
    unparsing_mode: Option<UnparsingMode>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(
        ctx: &'a TezosRpcContext<HttpClient>,
        big_map_id: u32,
        script_expr: &'a ScriptExprHash,
    ) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            big_map_id,
            script_expr,
            unparsing_mode: None,
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

    /// Normalize the script using an unparsing mode.
    ///
    /// Parsing modes:
    ///
    /// * [UnparsingMode::Readable]
    /// * [UnparsingMode::Optimized]
    /// * [UnparsingMode::Optimized_legacy]
    pub fn unparsing_mode(mut self, parsing_mode: UnparsingMode) -> Self {
        self.unparsing_mode = Some(parsing_mode);

        self
    }

    pub async fn send(&self) -> Result<Micheline, Error> {
        let mut path = self::path(
            self.chain_id.value(),
            self.block_id,
            self.big_map_id,
            self.script_expr.value(),
        );

        if self.unparsing_mode.is_none() {
            self.ctx.http_client().get(path.as_str()).await
        } else {
            path = format!("{}/normalized", path);

            let param = NormalizedPayload {
                unparsing_mode: self.unparsing_mode.unwrap(),
            };

            self.ctx
                .http_client()
                .post::<_, _, ()>(path.as_str(), &param, None)
                .await
        }
    }
}

/// Access the value associated with a key in a big map.
///
/// * `script_expr` - The Blake2b hash of the map key packed (Base58Check-encoded)
/// e.g. `expru3MJA26WX3kQ9WCPBPhCqsXE33BBtXnTQpYmQwtbJyHSu3ME9E`
///
/// [`GET /chains/<chain_id>/blocks/<block_id>/context/big_maps/<big_map_id>/<script_expr>`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-big-maps-big-map-id-script-expr)
///
/// If `unparsing_mode` is provided, the request below will be used.
///
/// [`POST /chains/<chain_id>/blocks/<block_id>/context/big_maps/<big_map_id>/<script_expr>/normalized`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-big-maps-big-map-id-script-expr-normalized)
pub fn get_or_post<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    big_map_id: u32,
    script_expr: &'a ScriptExprHash,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, big_map_id, script_expr)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use tezos_core::types::encoded::{Encoded, ScriptExprHash};

    use crate::{client::TezosRpcChainId, models::contract::UnparsingMode};

    use {
        crate::{client::TezosRpc, error::Error, protocol_rpc::block::BlockId},
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_big_map_value() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let value = include_str!("__TEST_DATA__/michelson_value.json");
        let big_map_id: u32 = 162771;
        let big_map_key: ScriptExprHash = "expru3MJA26WX3kQ9WCPBPhCqsXE33BBtXnTQpYmQwtbJyHSu3ME9E"
            .try_into()
            .unwrap();
        let block_id = BlockId::Level(100);

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                TezosRpcChainId::Main.value(),
                &block_id,
                big_map_id,
                big_map_key.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(value);
        });

        let client = TezosRpc::new(rpc_url);
        let big_map = client
            .get_big_map_value(big_map_id, &big_map_key)
            .block_id(&block_id)
            .send()
            .await?;

        assert!(big_map.is_primitive_application());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_big_map_normalized_value() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let value = include_str!("__TEST_DATA__/michelson_value.json");
        let big_map_id: u32 = 162771;
        let big_map_key: ScriptExprHash = "expru3MJA26WX3kQ9WCPBPhCqsXE33BBtXnTQpYmQwtbJyHSu3ME9E"
            .try_into()
            .unwrap();
        let block_id = BlockId::Level(100);

        let body = serde_json::to_string(&super::NormalizedPayload {
            unparsing_mode: UnparsingMode::Optimized,
        })?;

        server.mock(|when, then| {
            when.method(POST)
                .path(format!(
                    "{}/normalized",
                    super::path(
                        TezosRpcChainId::Main.value(),
                        &block_id,
                        big_map_id,
                        big_map_key.value(),
                    )
                ))
                .body(body);
            then.status(200)
                .header("content-type", "application/json")
                .body(value);
        });

        let client = TezosRpc::new(rpc_url);
        let big_map = client
            .get_big_map_value(big_map_id, &big_map_key)
            .block_id(&block_id)
            .unparsing_mode(UnparsingMode::Optimized)
            .send()
            .await?;

        assert!(big_map.is_primitive_application());

        Ok(())
    }
}
