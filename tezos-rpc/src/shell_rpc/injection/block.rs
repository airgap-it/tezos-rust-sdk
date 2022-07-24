use tezos_core::types::encoded::BlockHash;

use crate::{client::TezosRpcChainId, http::Http};

use {crate::client::TezosRpcContext, crate::error::Error, serde::Serialize};

fn path() -> String {
    format!("{}/block", super::path())
}

#[derive(Serialize)]
pub struct OperationPayload {
    /// A block identifier (Base58Check-encoded)
    pub branch: BlockHash,
    /// Signed operation data
    pub data: String,
}

/// [InjectionBlockPayload] is used in request [`POST /injection/block`](post)
#[derive(Serialize)]
pub struct InjectionBlockPayload {
    /// Signed block data
    data: String,
    /// Operations to be included in the block
    operations: Vec<Vec<OperationPayload>>,
}

/// A builder to construct the properties of a request to inject a block in the node and broadcast it.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    payload: &'a InjectionBlockPayload,
    force: Option<bool>,
    do_async: Option<bool>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, payload: &'a InjectionBlockPayload) -> Self {
        Self {
            ctx,
            chain_id: ctx.chain_id(),
            payload,
            force: None,
            do_async: None,
        }
    }

    /// Modify chain identifier to be used in the request.
    /// The `chain` query parameter can be used to specify whether to inject on the test chain or the main chain.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    /// If `force` query parameter is `true`, it will be injected even on non strictly increasing fitness.
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);

        self
    }

    /// If `async` query parameter is true, the function returns immediately. Otherwise, the block will be validated before the result is returned.
    pub fn do_async(mut self, do_async: bool) -> Self {
        self.do_async = Some(do_async);

        self
    }

    pub async fn send(&self) -> Result<String, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(do_async) = self.do_async {
            // Add `async` query parameter
            query.push(("async", do_async.to_string()));
        }
        if let Some(force) = self.force {
            // Add `force` query parameter
            query.push(("force", force.to_string()));
        }
        // Add `chain` query parameter
        query.push(("chain", self.ctx.chain_id().value().into()));

        self.ctx
            .http_client()
            .post(self::path().as_str(), self.payload, Some(&query))
            .await
    }
}

/// Inject a block in the node and broadcast it.
///
/// The `operations` might be pre-validated using a contextual RPCs
/// from the latest block (e.g. `/blocks/head/context/preapply`).
///
/// By default, the RPC will wait for the block to be validated before answering.
/// If `?async` is true, the function returns immediately. Otherwise, the block will be validated before the result is returned. If ?force is true, it will be injected even on non strictly increasing fitness. An optional ?chain parameter can be used to specify whether to inject on the test chain or the main chain.
///
/// Returns the ID of the block.
///
/// [`POST /injection/block?[async]&[force]&[chain=<chain_id>]]`](https://tezos.gitlab.io/shell/rpc.html#post-injection-block)
pub fn post<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    payload: &'a InjectionBlockPayload,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, payload)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use {
        super::{InjectionBlockPayload, OperationPayload},
        crate::client::TezosRpc,
        crate::error::Error,
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_block_injection() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_hash = "BLEpXjUTYFaow75TR53W4nJFWLfPy2xrYhmoCckrxELznS5uDA2";
        let payload = InjectionBlockPayload {
            data: "blahblahblah".into(),
            operations: vec![
                vec![OperationPayload {
                    branch: "BLLRYycW8GicK1MDEyT9rQNfgSx9utBjM5Pz3QNUNs6W8PTJY9c"
                        .try_into()
                        .unwrap(),
                    data: "blahblahblah".into(),
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

        let client = TezosRpc::new(rpc_url);
        let op_hash = client
            .inject_block(&payload)
            .force(false)
            .do_async(false)
            .send()
            .await?;

        assert_eq!(block_hash, op_hash);

        Ok(())
    }
}
