use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct Ed25519BlindedPublicKeyHash {
    base58: String,
}

impl Encoded for Ed25519BlindedPublicKeyHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Ed25519BlindedPublicKeyHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "btz1",
    base58_length: 37,
    bytes_prefix: &[1, 2, 49, 223],
    bytes_length: 20,
};

impl TryFrom<&Vec<u8>> for Ed25519BlindedPublicKeyHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Ed25519BlindedPublicKeyHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Ed25519BlindedPublicKeyHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ed25519BlindedPublicKeyHash::new(value)
    }
}

impl TryFrom<&str> for Ed25519BlindedPublicKeyHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ed25519BlindedPublicKeyHash::new(value.to_string())
    }
}

impl TryFrom<&Ed25519BlindedPublicKeyHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Ed25519BlindedPublicKeyHash) -> Result<Self> {
        value.to_bytes()
    }
}