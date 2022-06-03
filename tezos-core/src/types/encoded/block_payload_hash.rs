use crate::internal::coder::encoded::encoded_bytes_coder::EncodedBytesCoder;
use crate::internal::coder::Encoder;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct BlockPayloadHash {
    base58: String,
}

impl Encoded for BlockPayloadHash {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(BlockPayloadHash { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "vh",
    base58_length: 52,
    bytes_prefix: &[1, 106, 242],
    bytes_length: 32,
};

impl TryFrom<&Vec<u8>> for BlockPayloadHash {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for BlockPayloadHash {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for BlockPayloadHash {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        BlockPayloadHash::new(value)
    }
}

impl TryFrom<&str> for BlockPayloadHash {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        BlockPayloadHash::new(value.to_string())
    }
}

impl TryFrom<&BlockPayloadHash> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &BlockPayloadHash) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: BlockPayloadHash =
            "vh3ZMFvh79oP7WiYs4kUdirPE2UGVFDJPzWPHQWHic3zsTJurwTU".try_into()?;
        assert_eq!(
            value.base58(),
            "vh3ZMFvh79oP7WiYs4kUdirPE2UGVFDJPzWPHQWHic3zsTJurwTU"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                248, 3, 45, 179, 210, 181, 30, 158, 205, 50, 116, 10, 226, 52, 91, 157, 117, 205,
                238, 139, 146, 126, 105, 146, 174, 25, 28, 209, 238, 87, 4, 173
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: BlockPayloadHash = [
            248, 3, 45, 179, 210, 181, 30, 158, 205, 50, 116, 10, 226, 52, 91, 157, 117, 205, 238,
            139, 146, 126, 105, 146, 174, 25, 28, 209, 238, 87, 4, 173,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "vh3ZMFvh79oP7WiYs4kUdirPE2UGVFDJPzWPHQWHic3zsTJurwTU"
        );

        Ok(())
    }
}
