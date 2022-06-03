use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct BlockHash {
    base58: String,
}

impl<'a> Encoded for BlockHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(BlockHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "B",
    base58_length: 51,
    bytes_prefix: &[1, 52],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for BlockHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for BlockHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for BlockHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        BlockHash::new(value)
    }
}

impl TryFrom<&str> for BlockHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        BlockHash::new(value.to_string())
    }
}

impl TryFrom<&BlockHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &BlockHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: BlockHash = "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA".try_into()?;
        assert_eq!(
            value.base58(),
            "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                153, 103, 160, 148, 80, 29, 14, 26, 87, 234, 129, 153, 155, 172, 15, 6, 24, 44, 32,
                47, 78, 64, 97, 80, 51, 203, 69, 223, 229, 241, 173, 76
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: BlockHash = [
            153, 103, 160, 148, 80, 29, 14, 26, 87, 234, 129, 153, 155, 172, 15, 6, 24, 44, 32, 47,
            78, 64, 97, 80, 51, 203, 69, 223, 229, 241, 173, 76,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA"
        );

        Ok(())
    }
}
