use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tezos_core::validation::is_str;

use crate::{Error, Result};
use alloc::borrow::ToOwned;

/// A valid tezos String.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct String(alloc::string::String);

impl String {
    pub fn is_valid(value: &str) -> bool {
        is_str(value)
    }

    pub fn from_string(value: alloc::string::String) -> Result<Self> {
        if Self::is_valid(&value) {
            return Ok(Self(value));
        }
        Err(Error::InvalidStringValue)
    }

    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> alloc::string::String {
        self.0
    }
}

impl FromStr for String {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s.to_owned())
    }
}

impl TryFrom<alloc::string::String> for String {
    type Error = Error;

    fn try_from(value: alloc::string::String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for String {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.into())
    }
}

impl From<String> for alloc::string::String {
    fn from(value: String) -> Self {
        value.0
    }
}
