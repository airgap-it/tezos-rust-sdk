//! The `tezos-operation` crate comes with means to create Tezos operations, forge and sign them.
//!
//! ## Create
//!
//! To create an [UnsignedOperation](crate::operations::UnsignedOperation) or [SignedOperation](crate::operations::SignedOperation):
//!
//! ```rust
//! use tezos_operation::operations::{UnsignedOperation, SignedOperation, OperationContent, Transaction};
//!
//! let unsigned = UnsignedOperation::new(
//!     "BMdhifZkcb5i9D6FnBi19SSBjft3sYaeKDAsEBgbsRLPTihQQJU".try_into().expect("valid conversion to BlockHash"),
//!     vec![
//!         Transaction::new(
//!             "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into().expect("valid conversion to ImplicitAddress"), // source
//!             0u8.into(), // fee: should be calculated
//!             0u8.into(), // counter: should be fetched from the node (and increased).
//!             0u8.into(), // gas_limit: should be calculated
//!             0u8.into(), // storage_limit: should be calculated
//!             1000u16.into(), // amount to transfer
//!             "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to ImplicitAddress"), // destination
//!             None, // no additional parameters
//!         ).into(),
//!     ]
//! );
//!
//! let signed = SignedOperation::from(
//!     unsigned,
//!     "sigTAzhy1HsZDLNETmuf9RuinhXRb5jvmscjCoPPBujWZgFmCFLffku7JXYtu8aYQFVHnCUghmd4t39RuR6ANV76bCCYTR9u".try_into().expect("valid conversion to Signature")
//! );
//! ```
//!
//! ## Forge
//!
//! To forge an operation use the [Operation::to_forged_bytes](crate::operations::Operation::to_forged_bytes) method
//! from the [Operation](crate::operations::Operation) trait that both [UnsignedOperation](crate::operations::UnsignedOperation)
//! and [SignedOperation](crate::operations::SignedOperation) implement.
//! To unforge an operation use [UnsignedOperation::from_forged_bytes](crate::operations::UnsignedOperation::from_forged_bytes).
//!
//! ```rust
//! use tezos_operation::operations::{UnsignedOperation, Operation, Transaction};
//!
//! let operation = UnsignedOperation::new(
//!     "BMdhifZkcb5i9D6FnBi19SSBjft3sYaeKDAsEBgbsRLPTihQQJU".try_into().expect("valid conversion to BlockHash"),
//!     vec![
//!         Transaction::new(
//!             "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into().expect("valid conversion to ImplicitAddress"), // source
//!             0u8.into(), // fee: should be calculated
//!             0u8.into(), // counter: should be fetched from the node (and increased).
//!             0u8.into(), // gas_limit: should be calculated
//!             0u8.into(), // storage_limit: should be calculated
//!             1000u16.into(), // amount to transfer
//!             "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to ImplicitAddress"), // destination
//!             None, // no additional parameters
//!         ).into(),
//!     ]
//! );
//!
//! let forged_bytes = operation.to_forged_bytes().expect("valid conversion to forged bytes");
//! let unforged = UnsignedOperation::from_forged_bytes(forged_bytes).expect("valid conversion to UnsignedOperation");
//! ```
//!
//! ## Sign and Verify
//!
//! Sign an operation with the [UnsignedOperation::into_signed_operation](crate::operations::UnsignedOperation::into_signed_operation) method:
//!
//! ```rust
//! use tezos_core::types::encoded::Ed25519SecretKey;
//! use tezos_operation::operations::{UnsignedOperation, Transaction};
//!
//! let secret_key: Ed25519SecretKey = "edskRv7VyXGVZb8EsrR7D9XKUbbAQNQGtALP6QeB16ZCD7SmmJpzyeneJVg3Mq56YLbxRA1kSdAXiswwPiaVfR3NHGMCXCziuZ".try_into().expect("valid conversion to Ed25519SecretKey");
//! let unsigned = UnsignedOperation::new(
//!     "BMdhifZkcb5i9D6FnBi19SSBjft3sYaeKDAsEBgbsRLPTihQQJU".try_into().expect("valid conversion to BlockHash"),
//!     vec![
//!         Transaction::new(
//!             "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into().expect("valid conversion to ImplicitAddress"), // source
//!             0u8.into(), // fee: should be calculated
//!             0u8.into(), // counter: should be fetched from the node (and increased).
//!             0u8.into(), // gas_limit: should be calculated
//!             0u8.into(), // storage_limit: should be calculated
//!             1000u16.into(), // amount to transfer
//!             "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to ImplicitAddress"), // destination
//!             None, // no additional parameters
//!         ).into(),
//!     ]
//! );
//! # #[cfg(feature = "ed25519")]
//! let signed = unsigned.into_signed_operation(&secret_key.into()).expect("valid signed operation");
//! ```
//!
//! The signature can also be generated by calling [UnsignedOperation::sign](crate::operations::UnsignedOperation::sign):
//!
//! ```rust
//! use tezos_core::types::encoded::Ed25519SecretKey;
//! use tezos_operation::operations::{UnsignedOperation, Transaction};
//!
//! let secret_key: Ed25519SecretKey = "edskRv7VyXGVZb8EsrR7D9XKUbbAQNQGtALP6QeB16ZCD7SmmJpzyeneJVg3Mq56YLbxRA1kSdAXiswwPiaVfR3NHGMCXCziuZ".try_into().expect("valid conversion to Ed25519SecretKey");
//! let unsigned = UnsignedOperation::new(
//!     "BMdhifZkcb5i9D6FnBi19SSBjft3sYaeKDAsEBgbsRLPTihQQJU".try_into().expect("valid conversion to BlockHash"),
//!     vec![
//!         Transaction::new(
//!             "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into().expect("valid conversion to ImplicitAddress"), // source
//!             0u8.into(), // fee: should be calculated
//!             0u8.into(), // counter: should be fetched from the node (and increased).
//!             0u8.into(), // gas_limit: should be calculated
//!             0u8.into(), // storage_limit: should be calculated
//!             1000u16.into(), // amount to transfer
//!             "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to ImplicitAddress"), // destination
//!             None, // no additional parameters
//!         ).into(),
//!     ]
//! );
//! # #[cfg(feature = "ed25519")]
//! let signature = unsigned.sign(&secret_key.into()).expect("valid signature");
//! ```
//!
//! To verify the operation's signature call [SignedOperation:verify](crate::operations::SignedOperation::verify):
//!
//! ```rust
//! use tezos_core::types::encoded::Ed25519PublicKey;
//! use tezos_operation::operations::{SignedOperation, Transaction};
//!
//! let public_key: Ed25519PublicKey = "edpkttZKC51wemRqL2QxwpMnEKxWnbd35pq47Y6xsCHp5M1f7LN8NP".try_into().expect("valid conversion to Ed25519PublicKey");
//! let signed = SignedOperation::new(
//!     "BMdhifZkcb5i9D6FnBi19SSBjft3sYaeKDAsEBgbsRLPTihQQJU".try_into().expect("valid conversion to BlockHash"),
//!     vec![
//!         Transaction::new(
//!             "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into().expect("valid conversion to ImplicitAddress"), // source
//!             0u8.into(), // fee: should be calculated
//!             0u8.into(), // counter: should be fetched from the node (and increased).
//!             0u8.into(), // gas_limit: should be calculated
//!             0u8.into(), // storage_limit: should be calculated
//!             1000u16.into(), // amount to transfer
//!             "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to ImplicitAddress"), // destination
//!             None, // no additional parameters
//!         ).into(),
//!     ],
//!     "edsigtyqcyfipEAqFVexKahDjtQkzFgAkd9MroyYHnxxUVHAi4oSBJSezwiKaoNpH9NkY34cNuR4nrL6s8oPWVstQp9h2f7iGQF".try_into().expect("valid conversion to Signature")
//! );
//! # #[cfg(feature = "ed25519")]
//! let is_signature_valid = signed.verify(&public_key.into()).expect("verification completed without errors");
//! ```

pub mod block_header;
mod error;
pub mod internal;
pub mod operations;

pub use error::{Error, Result};
