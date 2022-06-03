use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct SaplingSpendingKey {
    base58: String,
}

impl Encoded for SaplingSpendingKey {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(SaplingSpendingKey { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "sask",
    base58_length: 241,
    bytes_prefix: &[11, 237, 20, 92],
    bytes_length: 169,
};

impl TryFrom<&Vec<u8>> for SaplingSpendingKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for SaplingSpendingKey {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for SaplingSpendingKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        SaplingSpendingKey::new(value)
    }
}

impl TryFrom<&str> for SaplingSpendingKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        SaplingSpendingKey::new(value.to_string())
    }
}

impl TryFrom<&SaplingSpendingKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &SaplingSpendingKey) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
