#![allow(dead_code)]

//! The `tezos-core` crate defines many common types and basic crypto primitives used by
//! the `tezos-michelson`, `tezos-operation`, `tezos-rpc` and `tezos-contract` crates.
//!
//! # Address
//!
//! It supports the **tz**, **KT** and **zet** addresses. The addresses, however, can't be used
//! as plain strings across the library. To improve typing and compile-time checks, a few wrapper address types have been
//! introduced:
//!
//! **tz**, **KT**
//! - [Address](crate::types::encoded::Address) (enum)
//!     - **tz**
//!       - [ImplicitAddress](crate::types::encoded::ImplicitAddress) (enum)
//!       - **tz1**
//!         - [Ed25519PublicKeyHash](crate::types::encoded::Ed25519PublicKeyHash)
//!       - **tz2**
//!         - [Secp256K1PublicKeyHash](crate::types::encoded::Secp256K1PublicKeyHash)
//!       - **tz3**
//!         - [P256PublicKeyHash](crate::types::encoded::P256PublicKeyHash)
//!     - **KT1**
//!         - [ContractAddress](crate::types::encoded::ContractAddress)
//!
//! **zet1**
//! - [SaplingAddress](crate::types::encoded::SaplingAddress)
//!
//! Different types may be required depending on the use case.
//!
//! ## `Address` (type)
//!
//! [Address](crate::types::encoded::Address) is the most general address type that covers all the **tz** and **KT** addresses.
//! It is an enum with associated values: `Address::Implicit(AddressImplicit)` and `Address::Originated(ContractAddress)`.
//!
//! Create an [Address](crate::types::encoded::Address) instance from a string value:
//!
//! ```rust
//! use tezos_core::types::encoded::Address;
//!
//! let tz1_address: Address = "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".try_into().expect("valid conversion to Address");
//! let tz2_address: Address = "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to Address");
//! let tz3_address: Address = "tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B".try_into().expect("valid conversion to Address");
//! let kt1_address: Address = "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into().expect("valid conversion to Address");
//! ```
//!
//! or use one of the actual `struct`s that represent an address:
//!
//! ```rust
//! use tezos_core::types::encoded::{Ed25519PublicKeyHash, Secp256K1PublicKeyHash, P256PublicKeyHash, ContractAddress};
//!
//! let tz1_address: Ed25519PublicKeyHash = "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".try_into().expect("valid conversion to Ed25519PublicKeyHash");
//! let tz2_address: Secp256K1PublicKeyHash = "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to Secp256K1PublicKeyHash");
//! let tz3_address: P256PublicKeyHash = "tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B".try_into().expect("valid conversion to P256PublicKeyHash");
//! let kt1_address: ContractAddress = "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into().expect("valid conversion to ContractAddress");
//! ```
//!
//! ### `ImplicitAddress` (type)
//!
//! [ImplicitAddress](crate::types::encoded::ImplicitAddress) is an address type that covers all the **tz** addresses.
//! It is an enum with associated values: `Address::TZ1(Ed25519PublicKeyHash)`, `Address::TZ2(Secp256K1PublicKeyHash)` and `Address::TZ3(P256PublicKeyHash)`.
//!
//! Create an [ImplicitAddress](crate::types::encoded::ImplicitAddress) instance from a string value:
//!
//! ```rust
//! use tezos_core::types::encoded::ImplicitAddress;
//!
//! let tz1_implicit_address: ImplicitAddress = "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".try_into().expect("valid conversion to ImplicitAddress");
//! let tz2_implicit_address: ImplicitAddress = "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to ImplicitAddress");
//! let tz3_implicit_address: ImplicitAddress = "tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B".try_into().expect("valid conversion to ImplicitAddress");
//! ```
//!
//! or use one of the actual `struct`s that represent an address:
//!
//! ```rust
//! use tezos_core::types::encoded::{Ed25519PublicKeyHash, Secp256K1PublicKeyHash, P256PublicKeyHash};
//!
//! let tz1_implicit_address: Ed25519PublicKeyHash = "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".try_into().expect("valid conversion to Ed25519PublicKeyHash");
//! let tz2_implicit_address: Secp256K1PublicKeyHash = "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().expect("valid conversion to Secp256K1PublicKeyHash");
//! let tz3_implicit_address: P256PublicKeyHash = "tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B".try_into().expect("valid conversion to P256PublicKeyHash");
//! ```
//!
//! [ImplicitAddress](crate::types::encoded::ImplicitAddress) can also be easily transformed to [Address](crate::types::encoded::Address):
//!
//! ```rust
//! use tezos_core::types::encoded::{Address, ImplicitAddress, Encoded};
//!
//! let tz1_address: Address = ImplicitAddress::new("tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".into()).unwrap().into();
//! let tz2_address: Address = ImplicitAddress::new("tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".into()).unwrap().into();
//! let tz3_address: Address = ImplicitAddress::new("tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B".into()).unwrap().into();
//! ```
//!
//! ### `ContractAddress` (type)
//!
//! [ContractAddress](crate::types::encoded::ContractAddress) is an address type that covers all the **KT** addresses.
//!
//! Create an [ContractAddress](crate::types::encoded::ContractAddress) instance from a string value:
//!
//! ```rust
//! use tezos_core::types::encoded::ContractAddress;
//!
//! let kt1_address: ContractAddress = "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into().expect("valid conversion to ContractAddress");
//! ```
//!
//! [ContractAddress](crate::types::encoded::ContractAddress) can also be easily transformed to `Address`:
//!
//! ```rust
//! use tezos_core::types::encoded::{ContractAddress, Address, Encoded};
//!
//! let kt1_address: Address = ContractAddress::new("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".into()).unwrap().into();
//! ```
//!
//! # Base58 Encoded Data
//!
//! Similarly to the addresses, the use of a plain string Base58 encoded data (i.e. string values whose data type can be
//! recognized by their Base58 prefix, e.g. **sig**, **Net**, **expr**, etc.) is not supported. The data must be first wrapped
//! in its responding type before it can be used in the library, for example:
//!
//! ```rust
//! use tezos_core::types::encoded::{BlockHash, ChainId, Ed25519PublicKey, Encoded};
//!
//! let block_hash = BlockHash::new("BLrUSnmhoWczorTYG8utWTLcD8yup6MX1MCehXG8f8QWew8t1N8".into()).unwrap();
//! let chain_id = ChainId::new("NetXPduhFKtb9SG".into()).unwrap();
//! let ed25519_public_key = Ed25519PublicKey::new("edpktmJqEE79FtfdWse1gqnUey1vNBkB3zNV99Pi95SRAs8NMatczG".into());
//! ```

pub mod crypto;
mod error;
pub mod internal;
pub mod types;
pub mod validation;

use cfg_if::cfg_if;

#[cfg(feature = "ed25519")]
use crate::crypto::default::DefaultEd25519CryptoProvider;
#[cfg(feature = "p256")]
use crate::crypto::default::DefaultP256CryptoProvider;
#[cfg(feature = "secp256_k1")]
use crate::crypto::default::DefaultSecp256K1CryptoProvider;
use crate::internal::crypto::Crypto;
pub use crate::{
    crypto::CryptoProvider,
    error::{Error, Result},
};

/// A structure used to provide configurations to other tezos crates.
///
/// So far, the only configurable aspect is the crypto primitives implementation defined the the `CryptoConfig` trait.
///
/// [Tezos] implements the [Default] trait providing a default implementation of the crypto primitives provided the corresponding features are enabled (`ed25519`, `secp256_k1`, `p256`).
///
/// # Example
///
/// The following example shows how to provide your own crypto providers:
///
/// ```rust
/// use tezos_core::{Tezos, CryptoProvider, CryptoConfig};
///
/// struct MyCryptoConfig;
///
/// impl CryptoConfig for MyCryptoConfig {
///     fn get_ed25519_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
///         todo!()
///     }
///
///     fn get_secp256_k1_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
///         todo!()
///     }
///
///     fn get_p256_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
///         todo!()
///     }
/// }
///
/// let tezos = Tezos::new(Box::new(MyCryptoConfig));
/// ```
pub struct Tezos {
    crypto_config: Box<dyn CryptoConfig>,
}

impl Tezos {
    /// Creates a new instance with the given crypto config.
    pub fn new(crypto_config: Box<dyn CryptoConfig>) -> Self {
        Tezos { crypto_config }
    }

    /// Returns a `Crypto` instance providing access to the various crypto providers.
    pub fn get_crypto(&self) -> Crypto {
        Crypto::new(
            self.crypto_config.get_ed25519_crypto_provider(),
            self.crypto_config.get_secp256_k1_crypto_provider(),
            self.crypto_config.get_p256_crypto_provider(),
        )
    }
}

/// Config trait used to provide the various crypto provider. See the description for `Tezos`.
pub trait CryptoConfig {
    /// Should provide an instance of a structure implementing the [CryptoProvider] trait that implements the trait for ed25519 curve.
    /// If `None` is returned, then the functionality is considered not available.
    fn get_ed25519_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>>;
    /// Should provide an instance of a structure implementing the [CryptoProvider] trait that implements the trait for secp256_k1 curve.
    /// If `None` is returned, then the functionality is considered not available.
    fn get_secp256_k1_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>>;
    /// Should provide an instance of a structure implementing the [CryptoProvider] trait that implements the trait for p256 curve.
    /// If `None` is returned, then the functionality is considered not available.
    fn get_p256_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>>;
}

/// A structure providing the default implementation of [CryptoConfig].
///
/// This structure will provide default implementations for the
/// various crypto provider if the correspoding feature is enabled (`ed25519`, `secp256_k1`, `p256` or `full_crypto` for all of them).
pub struct DefaultCryptoConfig;

impl CryptoConfig for DefaultCryptoConfig {
    fn get_ed25519_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
        {
            cfg_if! {
                if #[cfg(feature = "ed25519")] {
                    Some(Box::new(DefaultEd25519CryptoProvider))
                } else {
                    None
                }
            }
        }
    }

    fn get_secp256_k1_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
        {
            cfg_if! {
                if #[cfg(feature = "secp256_k1")] {
                    Some(Box::new(DefaultSecp256K1CryptoProvider))
                } else {
                    None
                }
            }
        }
    }

    fn get_p256_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
        {
            cfg_if! {
                if #[cfg(feature = "p256")] {
                    Some(Box::new(DefaultP256CryptoProvider))
                } else {
                    None
                }
            }
        }
    }
}

impl Default for Tezos {
    fn default() -> Self {
        Self::new(Box::new(DefaultCryptoConfig))
    }
}
