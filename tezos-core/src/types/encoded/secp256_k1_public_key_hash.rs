use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct Secp256K1PublicKeyHash {
    base58: String,
}

impl Secp256K1PublicKeyHash {
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

impl Encoded for Secp256K1PublicKeyHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(Secp256K1PublicKeyHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "tz2",
    base58_length: 36,
    bytes_prefix: &[6, 161, 161],
    bytes_length: 20,
};

impl TryFrom<&Vec<u8>> for Secp256K1PublicKeyHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for Secp256K1PublicKeyHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for Secp256K1PublicKeyHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Secp256K1PublicKeyHash::new(value)
    }
}

impl TryFrom<&str> for Secp256K1PublicKeyHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Secp256K1PublicKeyHash::new(value.to_string())
    }
}

impl TryFrom<&Secp256K1PublicKeyHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Secp256K1PublicKeyHash) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let address: Secp256K1PublicKeyHash = "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot".try_into()?;
        assert_eq!(address.base58(), "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot");
        assert_eq!(
            address.to_bytes()?,
            [
                146, 174, 203, 241, 51, 100, 160, 131, 26, 170, 172, 13, 238, 138, 117, 165, 40,
                48, 171, 55
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let address: Secp256K1PublicKeyHash = [
            146, 174, 203, 241, 51, 100, 160, 131, 26, 170, 172, 13, 238, 138, 117, 165, 40, 48,
            171, 55,
        ]
        .try_into()?;
        assert_eq!(address.base58(), "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot");

        Ok(())
    }
}
