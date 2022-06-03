use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

#[derive(Debug)]
pub struct P256PublicKey {
    base58: String,
}

impl P256PublicKey {
    pub fn is_valid_base58(value: &str) -> bool {
        META.is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        META.is_valid_bytes(value)
    }

    pub fn is_valid_prefixed_bytes(value: &[u8]) -> bool {
        META.is_valid_prefixed_bytes(value)
    }
}

impl Encoded for P256PublicKey {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(P256PublicKey { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "p2pk",
    base58_length: 55,
    bytes_prefix: &[3, 178, 139, 127],
    bytes_length: 33,
};

impl TryFrom<&Vec<u8>> for P256PublicKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for P256PublicKey {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for P256PublicKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        P256PublicKey::new(value)
    }
}

impl TryFrom<&str> for P256PublicKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        P256PublicKey::new(value.to_string())
    }
}

impl TryFrom<&P256PublicKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &P256PublicKey) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
