use super::META_ENCODED_VALUES;
use crate::{Error, Result};

/// A structure providing metadata for encoded values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaEncoded {
    pub base58_prefix: &'static str,
    pub base58_length: usize,
    bytes_prefix: &'static [u8],
    pub bytes_length: usize,
}

impl MetaEncoded {
    pub const fn new(
        base58_prefix: &'static str,
        base58_length: usize,
        bytes_prefix: &'static [u8],
        bytes_length: usize,
    ) -> Self {
        Self {
            base58_prefix,
            base58_length,
            bytes_prefix,
            bytes_length,
        }
    }

    pub fn version(&self) -> u8 {
        self.bytes_prefix[0]
    }

    pub fn bytes_prefix(&self) -> &'static [u8] {
        &self.bytes_prefix[1..]
    }

    pub fn versioned_bytes_prefix(&self) -> &'static [u8] {
        self.bytes_prefix
    }

    pub fn prefixed_bytes_length(&self) -> usize {
        self.bytes_length + self.versioned_bytes_prefix().len()
    }

    pub fn is_valid_base58(&self, value: &str) -> bool {
        value.starts_with(self.base58_prefix) && value.len() == self.base58_length
    }

    pub fn is_valid_bytes(&self, value: &[u8]) -> bool {
        value.len() == self.bytes_length || self.is_valid_prefixed_bytes(value)
    }

    pub fn is_valid_prefixed_bytes(&self, value: &[u8]) -> bool {
        value.starts_with(&self.versioned_bytes_prefix())
            && value.len() == self.prefixed_bytes_length()
    }

    pub fn is_valid_consumable_bytes(&self, value: &[u8]) -> bool {
        value.len() == self.bytes_length || self.is_valid_prefixed_consumable_bytes(value)
    }

    pub fn is_valid_prefixed_consumable_bytes(&self, value: &[u8]) -> bool {
        value.starts_with(&self.versioned_bytes_prefix())
            && value.len() >= (self.bytes_length + self.versioned_bytes_prefix().len())
    }

    pub fn recognize_base58(value: &str) -> Result<&'static MetaEncoded> {
        META_ENCODED_VALUES
            .iter()
            .find(|item| item.is_valid_base58(value))
            .map(|item| *item)
            .ok_or(Error::InvalidBase58EncodedData {
                description: value.into(),
            })
    }

    pub fn recognize_bytes(value: &[u8]) -> Result<&'static MetaEncoded> {
        META_ENCODED_VALUES
            .iter()
            .find(|item| item.is_valid_prefixed_bytes(value))
            .map(|item| *item)
            .ok_or(Error::InvalidBytes)
    }

    pub fn recognize_consumable_bytes(value: &[u8]) -> Result<&'static MetaEncoded> {
        META_ENCODED_VALUES
            .iter()
            .find(|item| item.is_valid_prefixed_consumable_bytes(value))
            .map(|item| *item)
            .ok_or(Error::InvalidBytes)
    }
}

pub trait TraitMetaEncoded {
    fn meta_value() -> &'static MetaEncoded;
}
