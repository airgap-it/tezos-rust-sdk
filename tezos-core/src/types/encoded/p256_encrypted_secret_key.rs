use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct P256EncryptedSecretKey {
    base58: String,
}

impl Encoded for P256EncryptedSecretKey {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(P256EncryptedSecretKey { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "p2esk",
    base58_length: 88,
    bytes_prefix: &[9, 48, 57, 115, 171],
    bytes_length: 56,
};

impl TryFrom<&Vec<u8>> for P256EncryptedSecretKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for P256EncryptedSecretKey {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for P256EncryptedSecretKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        P256EncryptedSecretKey::new(value)
    }
}

impl TryFrom<&str> for P256EncryptedSecretKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        P256EncryptedSecretKey::new(value.to_string())
    }
}

impl TryFrom<&P256EncryptedSecretKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &P256EncryptedSecretKey) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}
