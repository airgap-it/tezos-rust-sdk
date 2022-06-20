use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct OperationMetadataListListHash {
    base58: String,
}

impl Encoded for OperationMetadataListListHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(OperationMetadataListListHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "LLr",
    base58_length: 53,
    bytes_prefix: &[29, 159, 182],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for OperationMetadataListListHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for OperationMetadataListListHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for OperationMetadataListListHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        OperationMetadataListListHash::new(value)
    }
}

impl TryFrom<&str> for OperationMetadataListListHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        OperationMetadataListListHash::new(value.to_string())
    }
}

impl TryFrom<&OperationMetadataListListHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &OperationMetadataListListHash) -> Result<Self> {
        value.to_bytes()
    }
}
