#![allow(dead_code)]

pub mod crypto;
pub mod error;
mod internal;
pub mod types;

#[cfg(feature = "crypto")]
pub use crate::crypto::default::DefaultCryptoProvider;
pub use crate::crypto::CryptoProvider;
pub use crate::error::{Error, Result};
use crate::internal::crypto::Crypto;

pub trait Config {
    fn get_crypto_provider(&self) -> Box<dyn CryptoProvider>;
}

pub struct Tezos {
    config: Box<dyn Config>,
}

impl Tezos {
    pub fn new(config: Box<dyn Config>) -> Self {
        Tezos { config }
    }

    pub fn get_crypto(&self) -> Crypto {
        Crypto::new(self.config.get_crypto_provider())
    }
}

#[cfg(feature = "crypto")]
pub struct DefaultConfig;

#[cfg(feature = "crypto")]
impl DefaultConfig {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(feature = "crypto")]
impl Config for DefaultConfig {
    fn get_crypto_provider(&self) -> Box<dyn CryptoProvider> {
        Box::new(DefaultCryptoProvider::new())
    }
}

#[cfg(feature = "crypto")]
impl Default for Tezos {
    fn default() -> Self {
        Self::new(Box::new(DefaultConfig::new()))
    }
}
