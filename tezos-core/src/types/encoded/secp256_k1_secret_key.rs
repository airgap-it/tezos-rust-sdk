use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

#[derive(Debug)]
pub struct Secp256K1SecretKey {
    base58: String,
}

impl Secp256K1SecretKey {
    pub fn is_valid_base58(value: &str) -> bool {
        META.is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        META.is_valid_bytes(value)
    }
}

impl Encoded for Secp256K1SecretKey {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Secp256K1SecretKey { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "spsk",
    base58_length: 54,
    bytes_prefix: &[17, 162, 224, 201],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for Secp256K1SecretKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Secp256K1SecretKey {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Secp256K1SecretKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Secp256K1SecretKey::new(value)
    }
}

impl TryFrom<&str> for Secp256K1SecretKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Secp256K1SecretKey::new(value.to_string())
    }
}

impl TryFrom<&Secp256K1SecretKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Secp256K1SecretKey) -> Result<Self> {
        value.to_bytes()
    }
}
