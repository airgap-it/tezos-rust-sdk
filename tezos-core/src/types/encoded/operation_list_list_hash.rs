use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct OperationListListHash {
    base58: String,
}

impl Encoded for OperationListListHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(OperationListListHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "LLo",
    base58_length: 53,
    bytes_prefix: &[29, 159, 109],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for OperationListListHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for OperationListListHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for OperationListListHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        OperationListListHash::new(value)
    }
}

impl TryFrom<&str> for OperationListListHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        OperationListListHash::new(value.to_string())
    }
}

impl TryFrom<&OperationListListHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &OperationListListHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: OperationListListHash =
            "LLoZpN9vikYaszkBgE5dELmghpyskaXjhwDzUQ9zNX5ou2qXYsd4r".try_into()?;
        assert_eq!(
            value.base58(),
            "LLoZpN9vikYaszkBgE5dELmghpyskaXjhwDzUQ9zNX5ou2qXYsd4r"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                65, 18, 20, 6, 159, 96, 252, 51, 234, 134, 99, 127, 244, 54, 69, 93, 187, 225, 63,
                110, 148, 143, 4, 247, 186, 156, 179, 75, 96, 67, 210, 150
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: OperationListListHash = [
            65, 18, 20, 6, 159, 96, 252, 51, 234, 134, 99, 127, 244, 54, 69, 93, 187, 225, 63, 110,
            148, 143, 4, 247, 186, 156, 179, 75, 96, 67, 210, 150,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "LLoZpN9vikYaszkBgE5dELmghpyskaXjhwDzUQ9zNX5ou2qXYsd4r"
        );

        Ok(())
    }
}
