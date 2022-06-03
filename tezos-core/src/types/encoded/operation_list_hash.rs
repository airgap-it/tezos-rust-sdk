use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct OperationListHash {
    base58: String,
}

impl Encoded for OperationListHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(OperationListHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "Lo",
    base58_length: 52,
    bytes_prefix: &[133, 233],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for OperationListHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for OperationListHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for OperationListHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        OperationListHash::new(value)
    }
}

impl TryFrom<&str> for OperationListHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        OperationListHash::new(value.to_string())
    }
}

impl TryFrom<&OperationListHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &OperationListHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
