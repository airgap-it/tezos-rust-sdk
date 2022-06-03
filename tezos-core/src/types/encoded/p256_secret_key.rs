use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

#[derive(Debug)]
pub struct P256SecretKey {
    base58: String,
}

impl P256SecretKey {
    pub fn is_valid_base58(value: &str) -> bool {
        META.is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        META.is_valid_bytes(value)
    }
}

impl Encoded for P256SecretKey {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(P256SecretKey { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "p2sk",
    base58_length: 54,
    bytes_prefix: &[16, 81, 238, 189],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for P256SecretKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for P256SecretKey {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for P256SecretKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        P256SecretKey::new(value)
    }
}

impl TryFrom<&str> for P256SecretKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        P256SecretKey::new(value.to_string())
    }
}

impl TryFrom<&P256SecretKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &P256SecretKey) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
