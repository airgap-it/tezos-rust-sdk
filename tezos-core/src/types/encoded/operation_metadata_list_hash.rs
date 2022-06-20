use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct OperationMetadataListHash {
    base58: String,
}

impl Encoded for OperationMetadataListHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(OperationMetadataListHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "Lr",
    base58_length: 52,
    bytes_prefix: &[134, 39],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for OperationMetadataListHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for OperationMetadataListHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for OperationMetadataListHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        OperationMetadataListHash::new(value)
    }
}

impl TryFrom<&str> for OperationMetadataListHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        OperationMetadataListHash::new(value.to_string())
    }
}

impl TryFrom<&OperationMetadataListHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &OperationMetadataListHash) -> Result<Self> {
        value.to_bytes()
    }
}
