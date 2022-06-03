use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct CryptoboxPublicKeyHash {
    base58: String,
}

impl Encoded for CryptoboxPublicKeyHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(CryptoboxPublicKeyHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "id",
    base58_length: 30,
    bytes_prefix: &[153, 103],
    bytes_length: 16,
};

impl TryFrom<&Vec<u8>> for CryptoboxPublicKeyHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for CryptoboxPublicKeyHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for CryptoboxPublicKeyHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        CryptoboxPublicKeyHash::new(value)
    }
}

impl TryFrom<&str> for CryptoboxPublicKeyHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        CryptoboxPublicKeyHash::new(value.to_string())
    }
}

impl TryFrom<&CryptoboxPublicKeyHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &CryptoboxPublicKeyHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
