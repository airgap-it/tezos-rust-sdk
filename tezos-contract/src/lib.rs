mod contract;
mod error;
mod utils;

pub use contract::*;
pub use error::{Error, Result};

#[cfg(test)]
mod test {
    use httpmock::{Method::POST, MockServer};
    use tezos_core::types::{
        encoded::{Address, ContractHash, Encoded},
        number::Nat,
    };
    use tezos_michelson::{
        micheline::{self, Micheline},
        michelson::{
            data::{pair, sequence, try_string},
            ComparableTypePrimitive, DataPrimitive, Primitive, TypePrimitive,
        },
        MichelinePacker,
    };
    use tezos_operation::operations::Parameters;
    use tezos_rpc::client::TezosRpc;

    use super::*;

    #[tokio::test]
    async fn test_contract_big_map_value() -> Result<()> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let contract_address: ContractHash = "KT&1J4CiyWPmtFPXAjpgBezM5hoVHXHNzWBHK".try_into()?;
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
        let ledger =
            contract
                .storage()
                .big_maps()
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
                    0u8.into(),
                ]),
                None,
            )
            .await?
            .try_into()?;

        assert_eq!("164748675300576703", balance.to_str());
        Ok(())
    }

    #[tokio::test]
    async fn test_contract_call() -> Result<()> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let source: Address = "tz1YY1LvD6TFH4z74pvxPQXBjAKHE5tB5Q8f".try_into()?;
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
        let rpc = TezosRpc::new(rpc_url.into());
        let contract = rpc.contract_at(contract_address, None).await?;
        let partial_transaction = contract.call(
            "transfer".into(),
            vec![(
                "",
                sequence(vec![pair(vec![
                    try_string(source.value())?,
                    sequence(vec![pair(vec![
                        try_string("tz1agAtczEzZS8tV67KGF4urNqxfNCSPksiW")?,
                        0u8.into(),
                        100u8.into(),
                    ])]),
                ])]),
            )],
        )?;

        let destination: ContractHash = (partial_transaction.destination).try_into()?;
        assert_eq!(contract.address(), &destination);

        let parameters_value: Micheline =
            micheline::sequence(vec![micheline::primitive_application(Primitive::Data(
                DataPrimitive::Pair,
            ))
            .with_args(vec![
                micheline::try_string(source.value())?,
                micheline::sequence(vec![micheline::primitive_application(DataPrimitive::Pair)
                    .with_args(vec![
                        micheline::try_string("tz1agAtczEzZS8tV67KGF4urNqxfNCSPksiW")?,
                        micheline::primitive_application(DataPrimitive::Pair)
                            .with_args(vec![micheline::int(0), micheline::int(100)])
                            .into(),
                    ])
                    .into()]),
            ])
            .into()]);
        let parameters_schema: Micheline = micheline::primitive_application(TypePrimitive::List)
            .with_args(vec![micheline::primitive_application(TypePrimitive::Pair)
                .with_args(vec![
                    micheline::primitive_application(ComparableTypePrimitive::Address).into(),
                    micheline::primitive_application(TypePrimitive::List)
                        .with_args(vec![micheline::primitive_application(TypePrimitive::Pair)
                            .with_args(vec![
                                micheline::primitive_application(ComparableTypePrimitive::Address)
                                    .into(),
                                micheline::primitive_application(TypePrimitive::Pair)
                                    .with_args(vec![
                                        micheline::primitive_application(
                                            ComparableTypePrimitive::Nat,
                                        )
                                        .into(),
                                        micheline::primitive_application(
                                            ComparableTypePrimitive::Nat,
                                        )
                                        .into(),
                                    ])
                                    .into(),
                            ])
                            .into()])
                        .into(),
                ])
                .into()])
            .into();
        let parameters_value = MichelinePacker::pre_pack(parameters_value, &parameters_schema)?;
        let expected_parameters = Parameters::new("transfer".into(), parameters_value);
        assert_eq!(Some(expected_parameters), partial_transaction.parameters);

        Ok(())
    }
}
