use crate::crypto::CryptoProvider;
use crate::{Error, Result};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Crypto {
    ed25519_provider: Option<Box<dyn CryptoProvider>>,
    secp256_k1_provider: Option<Box<dyn CryptoProvider>>,
    p256_provider: Option<Box<dyn CryptoProvider>>,
}

impl Crypto {
    pub fn new(
        ed25519_provider: Option<Box<dyn CryptoProvider>>,
        secp256_k1_provider: Option<Box<dyn CryptoProvider>>,
        p256_provider: Option<Box<dyn CryptoProvider>>,
    ) -> Self {
        Self {
            ed25519_provider,
            secp256_k1_provider,
            p256_provider,
        }
    }

    pub fn blake2b(&self, message: &[u8], size: usize) -> Result<Vec<u8>> {
        blake2b(message, size)
    }

    pub fn sign_ed25519(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
        self.ed25519_provider
            .as_ref()
            .ok_or(Error::CryptoProviderNotSet)?
            .sign(message, secret)
    }

    pub fn verify_ed25519(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        self.ed25519_provider
            .as_ref()
            .ok_or(Error::CryptoProviderNotSet)?
            .verify(message, signature, public_key)
    }

    pub fn sign_secp256_k1(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
        self.secp256_k1_provider
            .as_ref()
            .ok_or(Error::CryptoProviderNotSet)?
            .sign(message, secret)
    }

    pub fn verify_secp256_k1(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        self.secp256_k1_provider
            .as_ref()
            .ok_or(Error::CryptoProviderNotSet)?
            .verify(message, signature, public_key)
    }

    pub fn sign_p256(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
        self.p256_provider
            .as_ref()
            .ok_or(Error::CryptoProviderNotSet)?
            .sign(message, secret)
    }

    pub fn verify_p256(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool> {
        self.p256_provider
            .as_ref()
            .ok_or(Error::CryptoProviderNotSet)?
            .verify(message, signature, public_key)
    }
}

pub fn blake2b(message: &[u8], size: usize) -> Result<Vec<u8>> {
    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };
    let mut hasher = Blake2bVar::new(size)?;
    hasher.update(message);
    let mut buf = Vec::<u8>::new();
    buf.resize(size, 0);
    hasher.finalize_variable(&mut buf)?;
    Ok(buf)
}
