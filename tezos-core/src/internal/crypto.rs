use crate::crypto::CryptoProvider;
use crate::Result;

pub struct Crypto {
    crypto_provider: Box<dyn CryptoProvider>,
}

impl Crypto {
    pub fn new(crypto_provider: Box<dyn CryptoProvider>) -> Self {
        Crypto { crypto_provider }
    }

    pub fn sha256(&self, message: &[u8]) -> Vec<u8> {
        self.crypto_provider.sha256(message)
    }

    pub fn blake2b(&self, message: &[u8], size: usize) -> Result<Vec<u8>> {
        self.crypto_provider.blake2b(message, size)
    }

    pub fn sign_ed25519(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
        self.crypto_provider.sign_ed25519(message, secret)
    }

    pub fn verify_ed25519(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        self.crypto_provider
            .verify_ed25519(message, signature, public_key)
    }

    pub fn sign_secp256_k1(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
        self.crypto_provider.sign_secp256_k1(message, secret)
    }

    pub fn verify_secp256_k1(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        self.crypto_provider
            .verify_secp256_k1(message, signature, public_key)
    }

    pub fn sign_p256(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
        self.crypto_provider.sign_p256(message, secret)
    }

    pub fn verify_p256(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool> {
        self.crypto_provider
            .verify_p256(message, signature, public_key)
    }
}
