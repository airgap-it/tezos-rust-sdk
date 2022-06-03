use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct BlockMetadataHash {
    base58: String,
}

impl Encoded for BlockMetadataHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(BlockMetadataHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "bm",
    base58_length: 52,
    bytes_prefix: &[234, 249],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for BlockMetadataHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for BlockMetadataHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for BlockMetadataHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        BlockMetadataHash::new(value)
    }
}

impl TryFrom<&str> for BlockMetadataHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        BlockMetadataHash::new(value.to_string())
    }
}

impl TryFrom<&BlockMetadataHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &BlockMetadataHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
