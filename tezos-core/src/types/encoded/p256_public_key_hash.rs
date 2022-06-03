use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct P256PublicKeyHash {
    base58: String,
}

impl P256PublicKeyHash {
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

impl Encoded for P256PublicKeyHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(P256PublicKeyHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "tz3",
    base58_length: 36,
    bytes_prefix: &[6, 161, 164],
    bytes_length: 20,
};

impl TryFrom<&Vec<u8>> for P256PublicKeyHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for P256PublicKeyHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for P256PublicKeyHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        P256PublicKeyHash::new(value)
    }
}

impl TryFrom<&str> for P256PublicKeyHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        P256PublicKeyHash::new(value.to_string())
    }
}

impl TryFrom<&P256PublicKeyHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &P256PublicKeyHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let address: P256PublicKeyHash = "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q".try_into()?;
        assert_eq!(address.base58(), "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q");
        assert_eq!(
            address.to_bytes()?,
            [
                236, 248, 123, 167, 175, 44, 31, 221, 193, 241, 39, 219, 102, 192, 143, 101, 63,
                141, 55, 63
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let address: P256PublicKeyHash = [
            236, 248, 123, 167, 175, 44, 31, 221, 193, 241, 39, 219, 102, 192, 143, 101, 63, 141,
            55, 63,
        ]
        .try_into()?;
        assert_eq!(address.base58(), "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q");

        Ok(())
    }
}
