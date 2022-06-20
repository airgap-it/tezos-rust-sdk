use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct OperationHash {
    base58: String,
}

impl Encoded for OperationHash {
    type Coder = EncodedBytesCoder;

    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(OperationHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "o",
    base58_length: 51,
    bytes_prefix: &[5, 116],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for OperationHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for OperationHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        <Self as Encoded>::Coder::decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for OperationHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        OperationHash::new(value)
    }
}

impl TryFrom<&str> for OperationHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        OperationHash::new(value.to_string())
    }
}

impl TryFrom<&OperationHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &OperationHash) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: OperationHash =
            "ooG169iWhv7vQccPGcB2EWeAjFWvxcrmQVCi4eWCviUTHeQuH24".try_into()?;
        assert_eq!(
            value.base58(),
            "ooG169iWhv7vQccPGcB2EWeAjFWvxcrmQVCi4eWCviUTHeQuH24"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                81, 67, 247, 201, 125, 8, 189, 62, 40, 197, 46, 38, 67, 145, 144, 95, 233, 123, 38,
                150, 214, 84, 97, 115, 22, 163, 84, 51, 118, 106, 80, 7
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: OperationHash = [
            81, 67, 247, 201, 125, 8, 189, 62, 40, 197, 46, 38, 67, 145, 144, 95, 233, 123, 38,
            150, 214, 84, 97, 115, 22, 163, 84, 51, 118, 106, 80, 7,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "ooG169iWhv7vQccPGcB2EWeAjFWvxcrmQVCi4eWCviUTHeQuH24"
        );

        Ok(())
    }
}
