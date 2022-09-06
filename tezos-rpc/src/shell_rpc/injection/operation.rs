use crate::{client::TezosRpcChainId, http::Http};

use tezos_core::types::encoded::OperationHash;
use {crate::client::TezosRpcContext, crate::error::Error};

fn path() -> String {
    format!("{}/operation", super::path())
}

/// A builder to construct the properties of a request to inject an operation in node and broadcast it.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    payload: &'a str,
    do_async: Option<bool>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, payload: &'a str) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            payload,
            do_async: None,
        }
    }

    /// Modify chain identifier to be used in the request.
    /// The `chain` query parameter can be used to specify whether to inject on the test chain or the main chain.
    pub fn chain_id(mut self, chain_id: &'a TezosRpcChainId) -> Self {
        self.chain_id = chain_id;

        self
    }

    /// If `async` query parameter is true, the function returns immediately. Otherwise, the block will be validated before the result is returned.
    pub fn do_async(mut self, do_async: bool) -> Self {
        self.do_async = Some(do_async);

        self
    }

    pub async fn send(&self) -> Result<OperationHash, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(do_async) = self.do_async {
            // Add `async` query parameter
            query.push(("async", do_async.to_string()));
        }
        // Add `chain` query parameter
        query.push(("chain", self.ctx.chain_id().value().into()));

        let operaton_hash: String = self
            .ctx
            .http_client()
            .post(self::path().as_str(), &self.payload, Some(&query))
            .await?;

        Ok(operaton_hash.try_into()?)
    }
}

/// Inject an operation in node and broadcast it.
///
/// The `signed_operation_contents` should be constructed using contextual RPCs
/// from the latest block and signed by the client.
///
/// The injection of the operation will apply it on the current mempool context.
/// This context may change at each operation injection or operation reception from peers.
///
/// By default, the RPC will wait for the operation to be (pre-)validated before returning.
/// However, if `?async` is true, the function returns immediately.
/// The optional `?chain` parameter can be used to specify whether to inject on the test chain or the main chain.
///
/// Returns the ID of the operation.
///
/// [`POST /injection/operation?[async]&[chain=<chain_id>]`](https://tezos.gitlab.io/shell/rpc.html#post-injection-operation)
pub fn post<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    signed_operation_contents: &'a str,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, signed_operation_contents)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use tezos_core::types::encoded::{Encoded, OperationHash};
    use {crate::client::TezosRpc, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_operation_injection() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let operation_hash: OperationHash = "ooG169iWhv7vQccPGcB2EWeAjFWvxcrmQVCi4eWCviUTHeQuH24"
            .try_into()
            .unwrap();
        let signed_operation_contents = "fcd40228f821b0183a73fc0553a69095a319858e718cbd636cd28fde99c14cad6d009f7f36d0241d3e6a82254216d7de5780aa67d8f9a205ee8f0b8f0bb0030000000000570200000052050107610362036205000764046200000004256164640462000000072572656d6f766505020200000028037a072e020000000803210346034c0350020000000c034c053e0362057000020350053d036d0342000000050200000000efaf7b675fdb1488c778efa72a3288a768c622601c6cda306056a86a2074f61951b8071d3ab75e09064dc0697457a2371cd0e27ffb6a7d868fbe51007e7d9f0f";

        server.mock(|when, then| {
            when.method(POST)
                .path(super::path())
                .query_param("chain", "main")
                .query_param("async", "false")
                .json_body(signed_operation_contents);
            then.status(200)
                .header("content-type", "application/json")
                .json_body(operation_hash.value());
        });

        let client = TezosRpc::new(rpc_url);
        let op_hash = client
            .inject_operation(signed_operation_contents)
            .do_async(false)
            .send()
            .await?;

        assert_eq!(operation_hash, op_hash);

        Ok(())
    }
}
