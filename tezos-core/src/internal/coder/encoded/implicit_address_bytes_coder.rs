use std::ops::Add;

use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::ConsumableList,
        types::{BytesTag, EncodedTag},
    },
    types::encoded::{self, ImplicitAddress, MetaEncoded},
    Error, Result,
};

use super::encoded_group_bytes_coder::{EncodedGroupBytesCoder, TagProvider};

pub struct ImplicitAddressBytesCoder;

impl Encoder<ImplicitAddress, Vec<u8>, Error> for ImplicitAddressBytesCoder {
    fn encode(value: &ImplicitAddress) -> Result<Vec<u8>> {
        EncodedGroupBytesCoder::<Self>::encode(&value)
    }
}

impl Decoder<ImplicitAddress, Vec<u8>, Error> for ImplicitAddressBytesCoder {
    fn decode(value: &Vec<u8>) -> Result<ImplicitAddress> {
        EncodedGroupBytesCoder::<Self>::decode(value)
    }
}

impl ConsumingDecoder<ImplicitAddress, u8, Error> for ImplicitAddressBytesCoder {
    fn decode_consuming(value: &mut Vec<u8>) -> Result<ImplicitAddress> {
        EncodedGroupBytesCoder::<Self>::decode_consuming(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ImplicitAddressTag {
    TZ1,
    TZ2,
    TZ3,
}

impl BytesTag for ImplicitAddressTag {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::TZ1 => &[0],
            Self::TZ2 => &[1],
            Self::TZ3 => &[2],
        }
    }
}

impl EncodedTag for ImplicitAddressTag {
    fn values() -> &'static [Self] {
        &[Self::TZ1, Self::TZ2, Self::TZ3]
    }

    fn meta(&self) -> &MetaEncoded {
        match self {
            Self::TZ1 => &encoded::META_ED25519_PUBLIC_KEY_HASH,
            Self::TZ2 => &encoded::META_SECP256_K1_PUBLIC_KEY_HASH,
            Self::TZ3 => &encoded::META_P256_PUBLIC_KEY_HASH,
        }
    }
}

impl Add<Vec<u8>> for ImplicitAddressTag {
    type Output = Vec<u8>;

    fn add(self, rhs: Vec<u8>) -> Self::Output {
        self.prefixed_to(&rhs)
    }
}

impl TagProvider for ImplicitAddressBytesCoder {
    type E = ImplicitAddress;
    type T = ImplicitAddressTag;

    fn tag_from_encoded(encoded: &Self::E) -> Option<Self::T> {
        Self::T::from_encoded(encoded)
    }

    fn tag_from_bytes(bytes: &[u8]) -> Option<Self::T> {
        Self::T::recognize(bytes)
    }

    fn tag_consuming(bytes: &mut Vec<u8>) -> Option<Self::T> {
        if let Some(tag) = Self::T::recognize_consumable(bytes) {
            let _ = bytes.consume_until(tag.value().len());
            return Some(tag);
        }

        None
    }
}