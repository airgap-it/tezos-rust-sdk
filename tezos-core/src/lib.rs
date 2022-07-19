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

pub trait Config {
    fn get_ed25519_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>>;
    fn get_secp256_k1_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>>;
    fn get_p256_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>>;
}

pub struct Tezos {
    config: Box<dyn Config>,
}

impl Tezos {
    pub fn new(config: Box<dyn Config>) -> Self {
        Tezos { config }
    }

    pub fn get_crypto(&self) -> Crypto {
        Crypto::new(
            self.config.get_ed25519_crypto_provider(),
            self.config.get_secp256_k1_crypto_provider(),
            self.config.get_p256_crypto_provider(),
        )
    }
}

pub struct DefaultConfig;

impl Config for DefaultConfig {
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
        Self::new(Box::new(DefaultConfig))
    }
}
