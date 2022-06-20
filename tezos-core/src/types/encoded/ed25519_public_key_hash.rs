use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct Ed25519PublicKeyHash {
    base58: String,
}

impl Ed25519PublicKeyHash {
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

impl Encoded for Ed25519PublicKeyHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Ed25519PublicKeyHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "tz1",
    base58_length: 36,
    bytes_prefix: &[6, 161, 159],
    bytes_length: 20,
};

impl TryFrom<&Vec<u8>> for Ed25519PublicKeyHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Ed25519PublicKeyHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Ed25519PublicKeyHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ed25519PublicKeyHash::new(value)
    }
}

impl TryFrom<&str> for Ed25519PublicKeyHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ed25519PublicKeyHash::new(value.to_string())
    }
}

impl TryFrom<&Ed25519PublicKeyHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Ed25519PublicKeyHash) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let address: Ed25519PublicKeyHash = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT".try_into()?;
        assert_eq!(address.base58(), "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT");
        assert_eq!(
            address.to_bytes()?,
            [
                22, 230, 73, 148, 194, 221, 189, 41, 54, 149, 182, 62, 76, 173, 224, 41, 211, 200,
                181, 227
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes_1() -> Result<()> {
        let address: Ed25519PublicKeyHash = [
            22u8, 230, 73, 148, 194, 221, 189, 41, 54, 149, 182, 62, 76, 173, 224, 41, 211, 200,
            181, 227,
        ]
        .try_into()?;
        assert_eq!(address.base58(), "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT");

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes_2() -> Result<()> {
        let bytes = [
            6u8, 161, 159, 22, 230, 73, 148, 194, 221, 189, 41, 54, 149, 182, 62, 76, 173, 224, 41,
            211, 200, 181, 227,
        ]
        .to_vec();
        let address: Ed25519PublicKeyHash = (&bytes).try_into()?;
        assert_eq!(address.base58(), "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT");

        Ok(())
    }
}
