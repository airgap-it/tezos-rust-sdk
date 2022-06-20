use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct SaplingAddress {
    base58: String,
}

impl Encoded for SaplingAddress {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(SaplingAddress { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "zet1",
    base58_length: 69,
    bytes_prefix: &[18, 71, 40, 223],
    bytes_length: 43,
};

impl TryFrom<&Vec<u8>> for SaplingAddress {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for SaplingAddress {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for SaplingAddress {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        SaplingAddress::new(value)
    }
}

impl TryFrom<&str> for SaplingAddress {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        SaplingAddress::new(value.to_string())
    }
}

impl TryFrom<&SaplingAddress> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &SaplingAddress) -> Result<Self> {
        value.to_bytes()
    }
}