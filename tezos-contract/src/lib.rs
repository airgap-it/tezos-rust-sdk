mod contract;
mod error;
mod utils;

pub use contract::*;
pub use error::{Error, Result};

#[cfg(test)]
mod test {
    use httpmock::{Method::POST, MockServer};
    use tezos_core::types::{
        encoded::{ContractHash, Encoded},
        number::Nat,
    };
    use tezos_michelson::michelson::data::{pair, try_string};
    use tezos_rpc::client::TezosRpc;

    use super::*;

    #[tokio::test]
    async fn test_contract_big_map_value() -> Result<()> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address: ContractHash = "KT1J4CiyWPmtFPXAjpgBezM5hoVHXHNzWBHK".try_into()?;
        server.mock(|when, then| {
            when.method(POST).path(format!(
                "/chains/main/blocks/head/context/contracts/{}/script/normalized",
                contract_address.value(),
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/contract.json"));
        });
        let rpc = TezosRpc::new(rpc_url);
        let contract = rpc.contract_at(contract_address, None).await?;
        let ledger = contract
            .storage
            .big_maps
            .get_by_name("ledger")
            .ok_or(Error::Internal {
                description: "Ledger not found".into(),
            })?;

        server.mock(|when, then| {
            when.method(POST).path(format!(
                "/chains/main/blocks/head/context/big_maps/{}/exprua97oRxvyAVsw5QEcWaELYNdeqsZUoFRMXTBuKUoG6MHtbV7C3/normalized", ledger.id
            ));
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("__TEST_DATA__/big_map_value.json"));
        });

        let balance: Nat = ledger
            .get_value(
                pair(vec![
                    try_string("tz1YY1LvD6TFH4z74pvxPQXBjAKHE5tB5Q8f")?,
                    0i8.into(),
                ]),
                None,
            )
            .await?
            .try_into()?;

        assert_eq!("164748675300576703", balance.to_str());
        Ok(())
    }
}
