use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct NonceHash {
    base58: String,
}

impl Encoded for NonceHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(NonceHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "nce",
    base58_length: 53,
    bytes_prefix: &[69, 220, 169],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for NonceHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for NonceHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for NonceHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        NonceHash::new(value)
    }
}

impl TryFrom<&str> for NonceHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        NonceHash::new(value.to_string())
    }
}

impl TryFrom<&NonceHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &NonceHash) -> Result<Self> {
        value.to_bytes()
    }
}
