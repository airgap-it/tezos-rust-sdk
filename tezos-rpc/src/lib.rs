//! The `tezos-rpc` crate allows to directly interact with a Tezos node. It provides the developers
//! with tools to easily make HTTP request to fetch the node's data or inject operations. Additionally, it can estimate
//! the minimum fee that is required to inject an operation.
//!
//! ## Interact with a Node
//!
//! Create a [TezosRpc](crate::client::TezosRpc) instance and use its interface to make calls to a node of your choice:
//!
//! ```rust
//! use tezos_core::types::encoded::Address;
//! use tezos_rpc::{client::TezosRpc, Result};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let block = rpc.get_block().send().await?;
//!     let address: Address = "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into()?;
//!     let balance = rpc.get_contract_balance(&address).send().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Estimate the Operation Fee and Inject
//!
//! You can estimate the minimum fee required for the operation to be injected with the
//! [TezosRpc::min_fee](crate::client::TezosRpc::min_fee) method, the it can be injected using [TezosRpc::inject_operation](crate::client::TezosRpc::inject_operation):
//!
//! ```rust
//! use num_bigint::BigUint;
//! use tezos_core::types::encoded::{ImplicitAddress, SecretKey};
//! use tezos_rpc::{client::TezosRpc, Result, models::block::BlockId};
//! use tezos_operation::operations::{UnsignedOperation, Transaction, SignedOperation};
//!
//! async fn example() -> Result<()> {
//!     let rpc = TezosRpc::new("https://testnet-tezos.giganode.io".into());
//!     let branch = rpc.get_block_hash()
//!                     .block_id(&BlockId::Level(-2))
//!                     .send().await?;
//!     let source: ImplicitAddress = "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into()?;
//!     let counter: BigUint = rpc.get_contract_counter(&source.clone().into())
//!                     .send()
//!                     .await? + 1u8;
//!     let operation = UnsignedOperation::new(
//!         branch,
//!         vec![
//!             Transaction::new(
//!                 source,
//!                 0u8.into(), // fee: will be populated by min_fee() call
//!                 counter.into(),
//!                 0u8.into(), // gas_limit: will be populated by min_fee() call
//!                 0u8.into(), // storage_limit: will be populated by min_fee() call
//!                 1000u16.into(), // amount to transfer
//!                 "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into()?, // destination
//!                 None, // no additional parameters
//!             ).into(),
//!         ]
//!     );
//!     let operation_with_fee = rpc.min_fee(operation, None).await?;
//!     let secret_key: SecretKey = "edskRv7VyXGVZb8EsrR7D9XKUbbAQNQGtALP6QeB16ZCD7SmmJpzyeneJVg3Mq56YLbxRA1kSdAXiswwPiaVfR3NHGMCXCziuZ".try_into()?;
//!     let signed_operation = operation_with_fee.into_signed_operation(&secret_key)?;
//!     let operation_hash = rpc.inject_operation(signed_operation.to_injectable_string()?.as_str()).send().await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod constants;
mod error;
pub mod http;
pub mod models;
pub mod protocol_rpc;
pub mod shell_rpc;

mod internal;
mod serde_utils;

pub use error::{Error, Result};
