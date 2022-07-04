use {crate::client::TezosRPCContext, crate::constants::BLOCK_HEAD_ALIAS, crate::error::Error};

fn path(chain_id: String, block_id: String, contract: String) -> String {
    format!("{}/balance", super::path(chain_id, block_id, contract))
}

/// A builder to construct the properties of a request to access the balance of a contract.
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: String,
    block_id: String,
    contract: String,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext, contract: &String) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: ctx.chain_id.to_string(),
            block_id: BLOCK_HEAD_ALIAS.to_string(),
            contract: contract.to_string(),
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &String) -> &mut Self {
        self.chain_id = chain_id.into();

        self
    }

    /// Modify the block identifier identifier to be used in the request.
    ///
    /// Default: [BLOCK_HEAD_ALIAS]
    pub fn block_id(&mut self, block_id: &String) -> &mut Self {
        self.block_id = block_id.into();

        self
    }

    pub async fn send(self) -> Result<u64, Error> {
        let path = self::path(self.chain_id, self.block_id, self.contract);

        let balance: String = self.ctx.http_client.get(path.as_str()).await?;

        Ok(balance.parse::<u64>()?)
    }
}

/// Access the balance of a contract.
///
/// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/balance`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-balance)
pub fn get<'a>(ctx: &'a TezosRPCContext, address: &String) -> RPCRequestBuilder<'a> {
    RPCRequestBuilder::new(ctx, address)
}

#[cfg(test)]
mod tests {
    use crate::constants::{BLOCK_HEAD_ALIAS, DEFAULT_CHAIN_ALIAS};

    use {crate::client::TezosRPC, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_balance() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address = "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL";
        let balance: u64 = 9999999999999999999;

        server.mock(|when, then| {
            when.method(GET).path(super::path(
                DEFAULT_CHAIN_ALIAS.to_string(),
                BLOCK_HEAD_ALIAS.to_string(),
                contract_address.to_string(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(format!("{}", balance));
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let balance = client.get_balance(&contract_address.to_string()).await?;
        assert_eq!(balance, balance);

        Ok(())
    }
}
