use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

#[derive(Debug)]
pub struct Ed25519PublicKey {
    base58: String,
}

impl Ed25519PublicKey {
    pub fn is_valid_base58(value: &str) -> bool {
        META.is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        META.is_valid_bytes(value)
    }

    pub fn is_valid_prefixed_bytes(value: &[u8]) -> bool {
        META.is_valid_prefixed_bytes(value)
    }
}

impl Encoded for Ed25519PublicKey {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Ed25519PublicKey { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "edpk",
    base58_length: 54,
    bytes_prefix: &[13, 15, 37, 217],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for Ed25519PublicKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Ed25519PublicKey {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Ed25519PublicKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ed25519PublicKey::new(value)
    }
}

impl TryFrom<&str> for Ed25519PublicKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ed25519PublicKey::new(value.to_string())
    }
}

impl TryFrom<&Ed25519PublicKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Ed25519PublicKey) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: Ed25519PublicKey =
            "edpkumVGEtDQgDAcMyB5FRn7UBLuhzg6D7aEnCDamXgjqGxavnsgvP".try_into()?;
        assert_eq!(
            value.base58(),
            "edpkumVGEtDQgDAcMyB5FRn7UBLuhzg6D7aEnCDamXgjqGxavnsgvP"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                148, 48, 194, 172, 143, 225, 64, 60, 108, 187, 238, 58, 152, 177, 159, 63, 59, 189,
                216, 157, 6, 89, 179, 235, 110, 65, 6, 165, 203, 228, 19, 81
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: Ed25519PublicKey = [
            148, 48, 194, 172, 143, 225, 64, 60, 108, 187, 238, 58, 152, 177, 159, 63, 59, 189,
            216, 157, 6, 89, 179, 235, 110, 65, 6, 165, 203, 228, 19, 81,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "edpkumVGEtDQgDAcMyB5FRn7UBLuhzg6D7aEnCDamXgjqGxavnsgvP"
        );

        Ok(())
    }
}
