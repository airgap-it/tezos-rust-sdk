use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct ContractHash {
    base58: String,
}

impl ContractHash {
    pub fn is_valid_base58(value: &str) -> bool {
        META.is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        META.is_valid_bytes(value)
    }
}

impl Encoded for ContractHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(ContractHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "KT1",
    base58_length: 36,
    bytes_prefix: &[2, 90, 121],
    bytes_length: 20,
};

impl TryFrom<&Vec<u8>> for ContractHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for ContractHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for ContractHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ContractHash::new(value)
    }
}

impl TryFrom<&str> for ContractHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        ContractHash::new(value.to_string())
    }
}

impl TryFrom<&ContractHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &ContractHash) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let address: ContractHash = "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo".try_into()?;
        assert_eq!(address.base58(), "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo");
        assert_eq!(
            address.to_bytes()?,
            [
                174, 39, 64, 233, 124, 78, 32, 128, 225, 6, 128, 60, 217, 114, 47, 117, 226, 161,
                27, 197
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let address: ContractHash = [
            174, 39, 64, 233, 124, 78, 32, 128, 225, 6, 128, 60, 217, 114, 47, 117, 226, 161, 27,
            197,
        ]
        .try_into()?;
        assert_eq!(address.base58(), "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo");

        Ok(())
    }
}
