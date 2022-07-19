pub mod default;

use crate::Result;

pub trait CryptoProvider {
    fn sign(&self, message: &[u8], secret: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool>;
}
