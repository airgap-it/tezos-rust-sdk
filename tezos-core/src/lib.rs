#![allow(dead_code)]

pub mod crypto;
mod error;
pub mod internal;
pub mod types;

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
/// [Tezos] implements the [Default] trait providing a defeault implementation of the crypto primitives provided the corresponding features are enabled (`ed25519`, `secp256_k1`, `p256`).
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
