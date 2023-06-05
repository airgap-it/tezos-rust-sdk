use crate::types::encoded::{Encoded, MetaEncoded};
use alloc::vec::Vec;

pub trait BytesTag {
    fn value(&self) -> &'static [u8];

    fn prefixed_to(&self, bytes: &[u8]) -> Vec<u8> {
        [self.value(), bytes].concat()
    }
}

pub trait EncodedTag: BytesTag + Sized + Copy {
    fn meta(&self) -> &MetaEncoded;
    fn values() -> &'static [Self];

    fn is_valid(&self, bytes: &[u8]) -> bool {
        bytes.starts_with(self.value()) && self.meta().is_valid_bytes(&bytes[self.value().len()..])
    }

    fn is_valid_consumable(&self, bytes: &[u8]) -> bool {
        bytes.starts_with(self.value()) && bytes.len() >= self.meta().bytes_length
    }

    fn recognize(bytes: &[u8]) -> Option<Self>
    where
        Self: 'static,
    {
        Self::values()
            .iter()
            .find(|item| item.is_valid(bytes))
            .map(|item| *item)
    }

    fn recognize_consumable(bytes: &[u8]) -> Option<Self>
    where
        Self: 'static,
    {
        Self::values()
            .iter()
            .find(|item| item.is_valid_consumable(bytes))
            .map(|item| *item)
    }

    fn from_encoded<E: Encoded>(value: &E) -> Option<Self>
    where
        Self: 'static,
    {
        Self::values()
            .iter()
            .find(|item| item.meta() == value.meta())
            .map(|item| *item)
    }
}
