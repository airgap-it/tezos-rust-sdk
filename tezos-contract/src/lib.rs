//! The `tezos-contract` crate exposes a Smart Contract handler whose purpose is to simplify the interaction with contract's
//! storage and code. The handler provides methods to read a contract's storage and prepare operations with parameters
//! matching the contract's code.
//!
//! You can get the handler for a contract using the `TezosRpc` client from the `tezos-rpc` crate:
//!
//! ```rust
//! use tezos_rpc::client::TezosRpc;
//! use tezos_contract::{Contract, ContractFetcher, Result};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let contract = rpc.contract_at("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into()?, None).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Read the Storage
//!
//! To read the contract's storage values with the handler:
//!
//! ```rust
//! use tezos_rpc::client::TezosRpc;
//! use tezos_contract::{Contract, ContractFetcher, Result};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let contract = rpc.contract_at("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into()?, None).await?;
//!     let total_supply = contract.storage().get_by_name("total_supply").unwrap();
//!     Ok(())
//! }
//! ```
//!
//! The [Contract::storage](crate::Contract::storage) method returns a [Storage](crate::Storage) structure
//! that can be used to retrieve values from the storage by their annotation name, with the [Storage::get_by_name](crate::Storage::get_by_name) method,
//! or by their position inside the structure, with the [Storage::get_at_index](crate::Storage::get_at_index) method.
//!
//! To better understand how to read data from [Storage](crate::Storage), let's assume the contract's storage is of a type
//! defined as follows:
//! ```json
//! {
//!   "prim": "pair",
//!   "args": [
//!     {
//!       "prim": "big_map",
//!       "args": [
//!         {
//!           "prim": "address"
//!         },
//!         {
//!           "prim": "nat",
//!           "annots": [
//!             ":balance"
//!           ]
//!         }
//!       ],
//!       "annots": [
//!         "%ledger"
//!       ]
//!     },
//!     {
//!       "prim": "pair",
//!       "args": [
//!         {
//!           "prim": "address",
//!           "annots": [
//!             "%admin"
//!           ]
//!         },
//!         {
//!           "prim": "pair",
//!           "args": [
//!             {
//!               "prim": "bool",
//!               "annots": [
//!                 "%paused"
//!               ]
//!             },
//!             {
//!               "prim": "nat",
//!               "annots": [
//!                 "%total_supply"
//!               ]
//!             }
//!           ]
//!         }
//!       ],
//!       "annots": ["%fields"]
//!     }
//!   ]
//! }
//! ```
//!
//! and the returned value in Micheline is:
//!
//! ```json
//! {
//!   "prim": "Pair",
//!   "args": [
//!     {
//!       "int": "51296"
//!     },
//!     {
//!       "prim": "Pair",
//!       "args": [
//!         {
//!           "bytes": "0000a7848de3b1fce76a7ffce2c7ce40e46be33aed7c"
//!         },
//!         {
//!           "prim": "Pair",
//!           "args": [
//!             {
//!               "prim": "True"
//!             },
//!             {
//!               "int": "20"
//!             }
//!           ]
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! The individual values from the storage could be read as below:
//! ```rust
//! use tezos_rpc::client::TezosRpc;
//! use tezos_contract::{Contract, ContractFetcher, Result};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let contract = rpc.contract_at("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into()?, None).await?;
//!     let ledger = contract.storage().get_by_name("ledger"); // { "int": "51296" }
//!     let admin = contract.storage().get_by_name("admin"); // { "bytes": "0000a7848de3b1fce76a7ffce2c7ce40e46be33aed7c" }
//!     let paused = contract.storage().get_by_name("paused"); // { "prim": "True" }
//!     let total_supply = contract.storage().get_by_name("total_supply").unwrap(); // { "int": "20" }
//!     Ok(())
//! }
//! ```
//!
//! ### Read a BigMap
//!
//! The [Storage](crate::Storage) structure exposes a big map handle to easily access the contract's big maps and fetch their values.
//!
//! ```rust
//! use tezos_michelson::michelson::data::try_string;
//! use tezos_rpc::client::TezosRpc;
//! use tezos_contract::{Contract, ContractFetcher, Result};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let contract = rpc.contract_at("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into()?, None).await?;
//!     let big_map = contract.storage().big_maps().get_by_name("ledger").unwrap();
//!     let big_map_value = big_map.get_value(&rpc, try_string("my_key").unwrap(), None).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Prepare a Contract Call
//!
//! To prepare a contract call with the contract handler:
//!
//! ```rust
//! use tezos_core::types::{encoded::ImplicitAddress, number::Nat};
//! use tezos_rpc::client::TezosRpc;
//! use tezos_michelson::michelson::data::{sequence, pair, try_string, int};
//! use tezos_contract::{Contract, ContractFetcher, Result, PartialTransaction};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let contract = rpc.contract_at("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into()?, None).await?;
//!     let partial_transaction: PartialTransaction = contract.call(
//!         "transfer".into(),
//!         vec![
//!             (
//!                 "",
//!                 sequence(vec![
//!                     pair(vec![
//!                         try_string("tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e").unwrap(),
//!                         sequence(vec![
//!                             pair(vec![
//!                                 try_string("tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c").unwrap(),
//!                                 pair(vec![int(0i8), int(100i8)]),
//!                             ]),
//!                         ]),
//!                     ]),
//!                 ])
//!             ),
//!         ]
//!     )?;
//!     let source: ImplicitAddress = "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".try_into().unwrap();
//!     let counter: Nat = (rpc.get_contract_counter(&source.clone().into())
//!                     .send()
//!                     .await? + 1u8).into();
//!     let transaction = partial_transaction.complete_with(
//!         source,
//!         counter,
//!         None,
//!         None,
//!     );
//!     Ok(())
//! }
//! ```

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
                &rpc,
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
