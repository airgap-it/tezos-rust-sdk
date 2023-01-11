use serde::{Deserialize, Serialize};
use tezos_core::types::encoded::{Address, Encoded};

use {
    crate::client::TezosRpcContext,
    crate::error::Error,
    crate::{client::TezosRpcChainId, http::Http, models::block::BlockId},
};

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, delegate: S) -> String {
    format!("{}/voting_info", super::path(chain_id, block_id, delegate))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VotingInfo {
    voting_power: String,
    remaining_proposals: u32,
}

/// A builder to construct the properties of a request to access the balance of a contract.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
    chain_id: &'a TezosRpcChainId,
    block_id: &'a BlockId,
    delegate: &'a Address,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>, delegate: &'a Address) -> Self {
        RpcRequestBuilder {
            ctx,
            chain_id: ctx.chain_id(),
            block_id: &BlockId::Head,
            delegate,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a TezosRpcChainId) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockId) -> &mut Self {
        self.block_id = block_id;

        self
    }

    pub async fn send(&self) -> Result<VotingInfo, Error> {
        let path = self::path(self.chain_id.value(), self.block_id, self.delegate.value());

        self.ctx.http_client().get(path.as_str()).await
    }
}

/// Returns the delegate info (e.g. voting power) found in the listings of the current voting period.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/delegates/<pkh>/voting_info`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-delegates-pkh-voting-info)
pub fn get<'a, HttpClient: Http>(
    ctx: &'a TezosRpcContext<HttpClient>,
    delegate: &'a Address,
) -> RpcRequestBuilder<'a, HttpClient> {
    RpcRequestBuilder::new(ctx, delegate)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use tezos_core::types::encoded::{Address, Encoded};

    use crate::client::TezosRpcChainId;

    use {
        crate::client::TezosRpc, crate::error::Error, crate::protocol_rpc::block::BlockId,
        httpmock::prelude::*,
    };

    #[tokio::test]
    async fn test_get_voting_info() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let delegate_address: Address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".try_into().unwrap();
        let expected_remaining_proposals = 20;
        let expected_voting_power = "11528562933742";
        let block_id = BlockId::Head;

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                TezosRpcChainId::Main.value(),
                &block_id,
                delegate_address.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/voting_info.json"));
        });

        let client = TezosRpc::new(rpc_url);
        let voting_info = client
            .voting_power(&delegate_address)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(voting_info.voting_power, expected_voting_power);
        assert_eq!(
            voting_info.remaining_proposals,
            expected_remaining_proposals
        );

        Ok(())
    }
}
