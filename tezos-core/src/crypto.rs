#[cfg(feature = "crypto")]
pub mod default;

use crate::Result;

pub trait CryptoProvider {
    fn sha256(&self, message: &[u8]) -> Vec<u8>;
    fn blake2b(&self, message: &[u8], size: usize) -> Result<Vec<u8>>;

    fn sign_ed25519(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>>;
    fn verify_ed25519(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool>;

    fn sign_secp256_k1(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>>;
    fn verify_secp256_k1(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool>;

    fn sign_p256(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>>;
    fn verify_p256(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool>;
}
