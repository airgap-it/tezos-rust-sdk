use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct ProtocolHash {
    base58: String,
}

impl Encoded for ProtocolHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(ProtocolHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "P",
    base58_length: 51,
    bytes_prefix: &[2, 170],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for ProtocolHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for ProtocolHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for ProtocolHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ProtocolHash::new(value)
    }
}

impl TryFrom<&str> for ProtocolHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        ProtocolHash::new(value.to_string())
    }
}

impl TryFrom<&ProtocolHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &ProtocolHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: ProtocolHash =
            "Psithaca2MLRFYargivpo7YvUr7wUDqyxrdhC5CQq78mRvimz6A".try_into()?;
        assert_eq!(
            value.base58(),
            "Psithaca2MLRFYargivpo7YvUr7wUDqyxrdhC5CQq78mRvimz6A"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                132, 36, 82, 12, 249, 187, 240, 164, 39, 112, 32, 77, 149, 220, 193, 241, 30, 64,
                79, 219, 62, 144, 200, 72, 80, 196, 207, 219, 80, 197, 196, 185
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: ProtocolHash = [
            132, 36, 82, 12, 249, 187, 240, 164, 39, 112, 32, 77, 149, 220, 193, 241, 30, 64, 79,
            219, 62, 144, 200, 72, 80, 196, 207, 219, 80, 197, 196, 185,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "Psithaca2MLRFYargivpo7YvUr7wUDqyxrdhC5CQq78mRvimz6A"
        );

        Ok(())
    }
}
