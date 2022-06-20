use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct ChainID {
    base58: String,
}

impl Encoded for ChainID {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(ChainID { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "Net",
    base58_length: 15,
    bytes_prefix: &[87, 82, 0],
    bytes_length: 4,
};

impl TryFrom<&Vec<u8>> for ChainID {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for ChainID {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for ChainID {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ChainID::new(value)
    }
}

impl TryFrom<&str> for ChainID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        ChainID::new(value.to_string())
    }
}

impl TryFrom<&ChainID> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &ChainID) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: ChainID = "NetXdQprcVkpaWU".try_into()?;
        assert_eq!(value.base58(), "NetXdQprcVkpaWU");
        assert_eq!(value.to_bytes()?, [122, 6, 167, 112]);

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: ChainID = [122, 6, 167, 112].try_into()?;
        assert_eq!(value.base58(), "NetXdQprcVkpaWU");

        Ok(())
    }
}
