use hex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tezos_core::validation::is_hex_str;

use crate::{Error, Result};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// A structure representing bytes.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bytes(
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "literal_bytes_serializer",
            deserialize_with = "literal_bytes_deserializer"
        )
    )]
    String,
);

impl Bytes {
    /// Returns the bytes as an hex string.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Returns true if the provided value represents a valid hex string, with or without 0x prefix.
    pub fn is_valid(value: &str) -> bool {
        is_hex_str(value)
    }

    pub fn from_string(value: String) -> Result<Self> {
        if Self::is_valid(&value) {
            let mut value = value;
            if !value.starts_with("0x") {
                value = format!("0x{}", value);
            }
            return Ok(Self(value));
        }
        Err(Error::InvalidHexString)
    }
}

impl TryFrom<String> for Bytes {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for Bytes {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_owned())
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        let hex = format!("0x{}", hex::encode(value));
        Bytes(hex)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        let hex = format!("0x{}", hex::encode(value));
        Bytes(hex)
    }
}

impl From<&Bytes> for Vec<u8> {
    fn from(value: &Bytes) -> Self {
        hex::decode(&value.0[2..]).unwrap()
    }
}

#[cfg(feature = "serde")]
fn literal_bytes_serializer<S>(value: &str, s: S) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.starts_with("0x") {
        return s.serialize_str(&value[2..]);
    }
    s.serialize_str(value)
}

#[cfg(feature = "serde")]
fn literal_bytes_deserializer<'de, D>(d: D) -> core::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(d)?;
    if !value.starts_with("0x") {
        return Ok(format!("0x{}", value));
    }

    Ok(value)
}
