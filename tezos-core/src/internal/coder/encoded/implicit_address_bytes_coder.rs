use core::ops::Add;

use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::ConsumableList,
        types::{BytesTag, EncodedTag},
    },
    types::encoded::{
        Ed25519PublicKeyHash, ImplicitAddress, MetaEncoded, P256PublicKeyHash,
        Secp256K1PublicKeyHash, TraitMetaEncoded,
    },
    Error, Result,
};
use alloc::vec::Vec;

use super::encoded_group_bytes_coder::{EncodedGroupBytesCoder, TagProvider};

pub struct ImplicitAddressBytesCoder;

impl Encoder<ImplicitAddress, Vec<u8>, Error> for ImplicitAddressBytesCoder {
    fn encode(value: &ImplicitAddress) -> Result<Vec<u8>> {
        EncodedGroupBytesCoder::<Self>::encode(&value)
    }
}

impl Decoder<ImplicitAddress, [u8], Error> for ImplicitAddressBytesCoder {
    fn decode(value: &[u8]) -> Result<ImplicitAddress> {
        EncodedGroupBytesCoder::<Self>::decode(value)
    }
}

impl ConsumingDecoder<ImplicitAddress, u8, Error> for ImplicitAddressBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<ImplicitAddress> {
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
            Self::TZ1 => Ed25519PublicKeyHash::meta_value(),
            Self::TZ2 => Secp256K1PublicKeyHash::meta_value(),
            Self::TZ3 => P256PublicKeyHash::meta_value(),
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

    fn tag_consuming<CL: ConsumableList<u8>>(bytes: &mut CL) -> Option<Self::T> {
        if let Some(tag) = Self::T::recognize_consumable(bytes.inner_value()) {
            let _ = bytes.consume_until(tag.value().len());
            return Some(tag);
        }

        None
    }
}
