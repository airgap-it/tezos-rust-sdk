use core::marker::PhantomData;

use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::ConsumableList,
        types::{BytesTag, EncodedTag},
    },
    types::encoded::Encoded,
    Error, Result,
};
use alloc::vec::Vec;

use super::encoded_bytes_coder::EncodedBytesCoder;

pub struct EncodedGroupBytesCoder<T: TagProvider> {
    tag_provider: PhantomData<T>,
}

impl<T: TagProvider> Encoder<T::E, Vec<u8>, Error> for EncodedGroupBytesCoder<T> {
    fn encode(value: &T::E) -> Result<Vec<u8>> {
        let tag = T::tag_from_encoded(value).ok_or(Error::InvalidEncodedValue)?;
        Ok(tag.prefixed_to(&EncodedBytesCoder::encode(value)?))
    }
}

impl<T: TagProvider> Decoder<T::E, [u8], Error> for EncodedGroupBytesCoder<T> {
    fn decode(value: &[u8]) -> Result<T::E> {
        let tag = T::tag_from_bytes(value).ok_or(Error::InvalidEncodedValue)?;
        EncodedBytesCoder::decode_with_meta(&value[tag.value().len()..], tag.meta())
    }
}

impl<T: TagProvider> ConsumingDecoder<T::E, u8, Error> for EncodedGroupBytesCoder<T> {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<T::E> {
        let tag = T::tag_consuming(value).ok_or(Error::InvalidEncodedValue)?;
        EncodedBytesCoder::decode_consuming_with_meta(value, tag.meta())
    }
}

pub trait TagProvider {
    type E: Encoded;
    type T: EncodedTag;

    fn tag_from_encoded(encoded: &Self::E) -> Option<Self::T>;
    fn tag_from_bytes(bytes: &[u8]) -> Option<Self::T>;
    fn tag_consuming<CL: ConsumableList<u8>>(bytes: &mut CL) -> Option<Self::T>;
}
