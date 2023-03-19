use core::ops::Add;

use crate::{
    internal::{
        coder::{
            encoded::encoded_group_bytes_coder::{EncodedGroupBytesCoder, TagProvider},
            ConsumingDecoder, Decoder, Encoder,
        },
        consumable_list::ConsumableList,
        types::{BytesTag, EncodedTag},
    },
    types::encoded::{
        Ed25519PublicKey, MetaEncoded, P256PublicKey, PublicKey, Secp256K1PublicKey,
        TraitMetaEncoded,
    },
    Error, Result,
};
use alloc::vec::Vec;

pub struct PublicKeyBytesCoder;

impl Encoder<PublicKey, Vec<u8>, Error> for PublicKeyBytesCoder {
    fn encode(value: &PublicKey) -> Result<Vec<u8>> {
        EncodedGroupBytesCoder::<Self>::encode(&value)
    }
}

impl Decoder<PublicKey, [u8], Error> for PublicKeyBytesCoder {
    fn decode(value: &[u8]) -> Result<PublicKey> {
        EncodedGroupBytesCoder::<Self>::decode(value)
    }
}

impl ConsumingDecoder<PublicKey, u8, Error> for PublicKeyBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<PublicKey> {
        EncodedGroupBytesCoder::<Self>::decode_consuming(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PublicKeyTag {
    EdPK,
    SpPK,
    P2PK,
}

impl PublicKeyTag {
    pub fn recognize(bytes: &[u8]) -> Option<Self> {
        Self::values()
            .iter()
            .find(|item| item.is_valid(bytes))
            .map(|item| *item)
    }
}

impl BytesTag for PublicKeyTag {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::EdPK => &[0],
            Self::SpPK => &[1],
            Self::P2PK => &[2],
        }
    }
}

impl EncodedTag for PublicKeyTag {
    fn values() -> &'static [Self] {
        &[Self::EdPK, Self::SpPK, Self::P2PK]
    }

    fn meta(&self) -> &MetaEncoded {
        match self {
            Self::EdPK => Ed25519PublicKey::meta_value(),
            Self::SpPK => Secp256K1PublicKey::meta_value(),
            Self::P2PK => P256PublicKey::meta_value(),
        }
    }
}

impl Add<Vec<u8>> for PublicKeyTag {
    type Output = Vec<u8>;

    fn add(self, rhs: Vec<u8>) -> Self::Output {
        self.prefixed_to(&rhs)
    }
}

impl TagProvider for PublicKeyBytesCoder {
    type E = PublicKey;
    type T = PublicKeyTag;

    fn tag_from_encoded(encoded: &Self::E) -> Option<Self::T> {
        Self::T::from_encoded(encoded)
    }

    fn tag_from_bytes(bytes: &[u8]) -> Option<Self::T> {
        Self::T::recognize(bytes)
    }

    fn tag_consuming<CL: ConsumableList<u8>>(bytes: &mut CL) -> Option<Self::T> {
        if let Some(tag) = Self::T::recognize_consumable(bytes.inner_value()) {
            let _ = bytes.consume_until(tag.value().len());
            return Some(tag);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::encoded::Encoded;

    #[test]
    fn test_encode_1() -> Result<()> {
        let key: PublicKey = "edpkuHhTYggbo1d3vRJTtoKy9hFnZGc8Vpr6qEzbZMXWV69odaM3a4".try_into()?;
        let bytes = PublicKeyBytesCoder::encode(&key)?;
        assert_eq!(
            bytes,
            [
                0, 85, 23, 40, 115, 207, 99, 179, 122, 110, 91, 78, 243, 5, 143, 225, 59, 209, 136,
                84, 25, 50, 87, 48, 202, 220, 89, 250, 26, 11, 223, 114, 115
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_1() -> Result<()> {
        let bytes = [
            0, 85, 23, 40, 115, 207, 99, 179, 122, 110, 91, 78, 243, 5, 143, 225, 59, 209, 136, 84,
            25, 50, 87, 48, 202, 220, 89, 250, 26, 11, 223, 114, 115,
        ]
        .to_vec();
        let key = PublicKeyBytesCoder::decode(&bytes)?;
        assert_eq!(
            key.value(),
            "edpkuHhTYggbo1d3vRJTtoKy9hFnZGc8Vpr6qEzbZMXWV69odaM3a4"
        );
        Ok(())
    }

    #[test]
    fn test_encode_2() -> Result<()> {
        let key: PublicKey =
            "sppkCVP3G6y4SsGAiHdR8UUd9dpawhAMpe5RT87F8wHKT7izLgrUncF".try_into()?;
        let bytes = PublicKeyBytesCoder::encode(&key)?;
        assert_eq!(
            bytes,
            [
                1, 149, 43, 21, 1, 112, 113, 142, 177, 22, 32, 186, 217, 28, 154, 108, 113, 150,
                202, 125, 82, 150, 69, 83, 44, 112, 143, 51, 221, 107, 88, 85, 82, 245
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_2() -> Result<()> {
        let bytes = [
            1, 149, 43, 21, 1, 112, 113, 142, 177, 22, 32, 186, 217, 28, 154, 108, 113, 150, 202,
            125, 82, 150, 69, 83, 44, 112, 143, 51, 221, 107, 88, 85, 82, 245,
        ]
        .to_vec();
        let key = PublicKeyBytesCoder::decode(&bytes)?;
        assert_eq!(
            key.value(),
            "sppkCVP3G6y4SsGAiHdR8UUd9dpawhAMpe5RT87F8wHKT7izLgrUncF"
        );
        Ok(())
    }

    #[test]
    fn test_encode_3() -> Result<()> {
        let key: PublicKey =
            "p2pkE3k5ZLRUvXTtjqGesGCZQBQjPE1cZghFFAmZTeQm7WNTwfsqeZg".try_into()?;
        let bytes = PublicKeyBytesCoder::encode(&key)?;
        assert_eq!(
            bytes,
            [
                2, 240, 69, 207, 48, 150, 227, 132, 60, 58, 181, 151, 136, 192, 198, 214, 6, 9,
                203, 91, 212, 166, 141, 35, 172, 23, 145, 189, 122, 166, 43, 132, 29, 36
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_3() -> Result<()> {
        let bytes = [
            2, 240, 69, 207, 48, 150, 227, 132, 60, 58, 181, 151, 136, 192, 198, 214, 6, 9, 203,
            91, 212, 166, 141, 35, 172, 23, 145, 189, 122, 166, 43, 132, 29, 36,
        ]
        .to_vec();
        let key = PublicKeyBytesCoder::decode(&bytes)?;
        assert_eq!(
            key.value(),
            "p2pkE3k5ZLRUvXTtjqGesGCZQBQjPE1cZghFFAmZTeQm7WNTwfsqeZg"
        );
        Ok(())
    }
}
