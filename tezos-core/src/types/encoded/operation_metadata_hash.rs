use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct OperationMetadataHash {
    base58: String,
}

impl Encoded for OperationMetadataHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(OperationMetadataHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "r",
    base58_length: 51,
    bytes_prefix: &[5, 183],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for OperationMetadataHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for OperationMetadataHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for OperationMetadataHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        OperationMetadataHash::new(value)
    }
}

impl TryFrom<&str> for OperationMetadataHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        OperationMetadataHash::new(value.to_string())
    }
}

impl TryFrom<&OperationMetadataHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &OperationMetadataHash) -> Result<Self> {
        value.to_bytes()
    }
}
