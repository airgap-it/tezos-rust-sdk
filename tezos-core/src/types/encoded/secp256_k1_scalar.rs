use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct Secp256K1Scalar {
    base58: String,
}

impl Encoded for Secp256K1Scalar {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Secp256K1Scalar { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "SSp",
    base58_length: 53,
    bytes_prefix: &[38, 248, 136],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for Secp256K1Scalar {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Secp256K1Scalar {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Secp256K1Scalar {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Secp256K1Scalar::new(value)
    }
}

impl TryFrom<&str> for Secp256K1Scalar {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Secp256K1Scalar::new(value.to_string())
    }
}

impl TryFrom<&Secp256K1Scalar> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Secp256K1Scalar) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
