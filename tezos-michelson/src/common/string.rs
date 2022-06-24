use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    michelson::data::Data as MichelsonData,
    {Error, Result},
};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct String(std::string::String);

impl String {
    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new("^(\"|\r|\n|\t|\\b|\\\\|[^\"\\\\])*$").unwrap();
        re.is_match(value)
    }

    pub fn from_string(value: std::string::String) -> Result<Self> {
        if Self::is_valid(&value) {
            return Ok(Self(value));
        }
        Err(Error::InvalidStringValue)
    }

    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> std::string::String {
        self.0
    }
}

impl FromStr for String {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s.to_owned())
    }
}

impl TryFrom<std::string::String> for String {
    type Error = Error;

    fn try_from(value: std::string::String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for String {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_owned())
    }
}

impl From<String> for std::string::String {
    fn from(value: String) -> Self {
        value.0
    }
}

impl From<String> for MichelsonData {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl TryFrom<MichelsonData> for String {
    type Error = Error;

    fn try_from(value: MichelsonData) -> Result<Self> {
        if let MichelsonData::String(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}
