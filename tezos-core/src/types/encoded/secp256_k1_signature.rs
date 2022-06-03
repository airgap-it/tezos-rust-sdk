use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

use super::GenericSignature;

#[derive(Debug)]
pub struct Secp256K1Signature {
    base58: String,
}

impl Secp256K1Signature {
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

impl Encoded for Secp256K1Signature {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Secp256K1Signature { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "spsig1",
    base58_length: 99,
    bytes_prefix: &[13, 115, 101, 19, 63],
    bytes_length: 64,
};

impl TryFrom<&Vec<u8>> for Secp256K1Signature {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Secp256K1Signature {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Secp256K1Signature {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Secp256K1Signature::new(value)
    }
}

impl TryFrom<&str> for Secp256K1Signature {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Secp256K1Signature::new(value.to_string())
    }
}

impl TryFrom<&GenericSignature> for Secp256K1Signature {
    type Error = Error;

    fn try_from(value: &GenericSignature) -> Result<Self> {
        let bytes = value.to_bytes()?;
        (&bytes).try_into()
    }
}

impl TryFrom<&Secp256K1Signature> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Secp256K1Signature) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
