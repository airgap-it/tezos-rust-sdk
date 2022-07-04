use {
    crate::client::TezosRPCContext, crate::error::Error, serde::Serialize,
    tezos_core::types::encoded::BlockHash,
};

fn path() -> String {
    format!("{}{}", super::path(), "/block")
}

/// `InjectionBlockPayload` used in request [`POST /injection/block`](post)
#[derive(Serialize)]
pub struct InjectionBlockPayload {
    /// Signed block data
    data: String,
    /// Operations to be included in the block
    operations: Vec<Vec<OperationPayload>>,
}

#[derive(Serialize)]
pub struct OperationPayload {
    /// A block identifier (Base58Check-encoded)
    pub branch: BlockHash,
    /// Signed operation data
    pub data: String,
}

/// Inject a block in the node and broadcast it.
///
/// The `operations` might be pre-validated using a contextual RPCs
/// from the latest block (e.g. `/blocks/head/context/preapply`).
///
/// By default, the RPC will wait for the block to be validated before answering.
/// If `?async` is true, the function returns immediately. Otherwise, the block will be validated before the result is returned. If ?force is true, it will be injected even on non strictly increasing fitness. An optional ?chain parameter can be used to specify whether to inject on the test chain or the main chain.
///
/// Returns the ID of the block [BlockHash].
///
/// [`POST /injection/block?[async]&[force]&[chain=<chain_id>]]`](https://tezos.gitlab.io/shell/rpc.html#post-injection-block)
pub async fn post(
    ctx: &TezosRPCContext,
    payload: &InjectionBlockPayload,
    force: &bool,
    do_async: &bool,
) -> Result<BlockHash, Error> {
    let mut query: Vec<(&str, String)> = vec![];

    // Add `async` query parameter
    query.push(("async", do_async.to_string()));
    // Add `force` query parameter
    query.push(("force", force.to_string()));
    // Add `chain` query parameter
    query.push(("chain", ctx.chain_id.to_string()));

    ctx.http_client
        .post(self::path().as_str(), payload, &Some(query))
        .await
}

#[cfg(test)]
mod tests {
    use tezos_core::types::encoded::Encoded;

    use super::{InjectionBlockPayload, OperationPayload};

    use {crate::client::TezosRPC, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_block_injection() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_hash = "BLEpXjUTYFaow75TR53W4nJFWLfPy2xrYhmoCckrxELznS5uDA2";
        let payload = InjectionBlockPayload {
            data: "blahblahblah".to_string(),
            operations: vec![
                vec![OperationPayload {
                    branch: "BLLRYycW8GicK1MDEyT9rQNfgSx9utBjM5Pz3QNUNs6W8PTJY9c".try_into()?,
                    data: "blahblahblah".to_string(),
                }],
                vec![],
                vec![],
                vec![],
            ],
        };

        server.mock(|when, then| {
            when.method(POST)
                .path(super::path())
                .query_param("chain", "main")
                .query_param("async", "false")
                .query_param("force", "false")
                .json_body(serde_json::json!(payload));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(block_hash);
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let op_hash = client.inject_block(&payload, &false, &false).await?;

        assert_eq!(block_hash, op_hash.into_string());

        Ok(())
    }
}
