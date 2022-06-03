use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        types::{BytesTag, EncodedTag},
    },
    types::encoded::Encoded,
    Error, Result,
};

use super::encoded_bytes_coder::EncodedBytesCoder;

pub struct EncodedGroupBytesCoder<T: TagProvider> {
    coder: EncodedBytesCoder,
    tag_provider: T,
}

impl<T: TagProvider> EncodedGroupBytesCoder<T> {
    pub fn new(tag_provider: T) -> Self {
        EncodedGroupBytesCoder {
            coder: EncodedBytesCoder::new(),
            tag_provider,
        }
    }
}

impl<T: TagProvider> Encoder<&T::E, Vec<u8>> for EncodedGroupBytesCoder<T> {
    fn encode(&self, value: &T::E) -> Result<Vec<u8>> {
        let tag = self
            .tag_provider
            .tag_from_encoded(value)
            .ok_or(Error::InvalidEncodedValue)?;
        Ok(tag.prefixed_to(&self.coder.encode(value)?))
    }
}

impl<T: TagProvider> Decoder<T::E, &[u8]> for EncodedGroupBytesCoder<T> {
    fn decode(&self, value: &[u8]) -> Result<T::E> {
        let tag = self
            .tag_provider
            .tag_from_bytes(value)
            .ok_or(Error::InvalidEncodedValue)?;
        self.coder
            .decode_with_meta(&value[tag.value().len()..], tag.meta())
    }
}

impl<T: TagProvider> ConsumingDecoder<T::E, u8> for EncodedGroupBytesCoder<T> {
    fn decode_consuming(&self, value: &mut Vec<u8>) -> Result<T::E> {
        let tag = self
            .tag_provider
            .tag_consuming(value)
            .ok_or(Error::InvalidEncodedValue)?;
        self.coder.decode_consuming_with_meta(value, tag.meta())
    }
}

pub trait TagProvider {
    type E: Encoded;
    type T: EncodedTag;

    fn tag_from_encoded(&self, encoded: &Self::E) -> Option<Self::T>;
    fn tag_from_bytes(&self, bytes: &[u8]) -> Option<Self::T>;
    fn tag_consuming(&self, bytes: &mut Vec<u8>) -> Option<Self::T>;
}
