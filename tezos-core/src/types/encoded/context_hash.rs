use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct ContextHash {
    base58: String,
}

impl Encoded for ContextHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(ContextHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "Co",
    base58_length: 52,
    bytes_prefix: &[79, 199],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for ContextHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for ContextHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for ContextHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ContextHash::new(value)
    }
}

impl TryFrom<&str> for ContextHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        ContextHash::new(value.to_string())
    }
}

impl TryFrom<&ContextHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &ContextHash) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: ContextHash =
            "CoUiATua7N2jitdscnVnqDpmfnqwwiJyCZbco6qfmcykVmGwPLbY".try_into()?;
        assert_eq!(
            value.base58(),
            "CoUiATua7N2jitdscnVnqDpmfnqwwiJyCZbco6qfmcykVmGwPLbY"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                8, 193, 4, 139, 153, 132, 175, 196, 86, 32, 232, 16, 131, 217, 241, 254, 44, 4,
                138, 199, 118, 31, 137, 238, 8, 51, 223, 86, 180, 53, 103, 2
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: ContextHash = [
            8, 193, 4, 139, 153, 132, 175, 196, 86, 32, 232, 16, 131, 217, 241, 254, 44, 4, 138,
            199, 118, 31, 137, 238, 8, 51, 223, 86, 180, 53, 103, 2,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "CoUiATua7N2jitdscnVnqDpmfnqwwiJyCZbco6qfmcykVmGwPLbY"
        );

        Ok(())
    }
}
