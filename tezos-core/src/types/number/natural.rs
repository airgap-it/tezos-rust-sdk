use std::{fmt::Debug, str::FromStr};

use num_bigint::{BigUint, ToBigUint};
use num_traits::{Num, Unsigned};
use regex::Regex;

use crate::{
    internal::coder::{number::natural::NaturalBytesCoder, Decoder, Encoder},
    types::mutez::Mutez,
    Error, Result,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Natural(String);

impl Natural {
    pub fn from_string(value: String) -> Result<Self> {
        if Self::is_valid(&value) {
            return Ok(Natural(value));
        }
        Err(Error::InvalidUnsignedIntegerString)
    }

    pub fn from_integer<I: Unsigned + ToString>(value: I) -> Self {
        Self::from_string(value.to_string()).unwrap()
    }

    pub fn to_integer<I: Unsigned + FromStr>(&self) -> I
    where
        <I as FromStr>::Err: Debug,
    {
        self.0.parse::<I>().unwrap()
    }

    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new(r"^[0-9]+$").unwrap();
        re.is_match(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let coder = NaturalBytesCoder::new();
        coder.encode(self)
    }
}

impl ToString for Natural {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl ToBigUint for Natural {
    fn to_biguint(&self) -> Option<BigUint> {
        BigUint::from_str_radix(&self.0, 10)
            .map(Some)
            .unwrap_or(None)
    }
}

impl From<u8> for Natural {
    fn from(value: u8) -> Self {
        Self::from_integer(value)
    }
}

impl From<u16> for Natural {
    fn from(value: u16) -> Self {
        Self::from_integer(value)
    }
}

impl From<u32> for Natural {
    fn from(value: u32) -> Self {
        Self::from_integer(value)
    }
}

impl From<u64> for Natural {
    fn from(value: u64) -> Self {
        Self::from_integer(value)
    }
}

impl From<u128> for Natural {
    fn from(value: u128) -> Self {
        Self::from_integer(value)
    }
}

impl From<BigUint> for Natural {
    fn from(value: BigUint) -> Self {
        Self::from_integer(value)
    }
}

impl From<&Mutez> for Natural {
    fn from(mutez: &Mutez) -> Self {
        Self::from_integer(mutez.value())
    }
}

impl TryFrom<String> for Natural {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for Natural {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_string())
    }
}

impl TryFrom<&Vec<u8>> for Natural {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = NaturalBytesCoder::new();
        coder.decode(value)
    }
}

impl TryFrom<&Natural> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Natural) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_naturals() -> Result<()> {
        let values = vec![
            "0",
            "1",
            "127",
            "32767",
            "2147483647",
            "9223372036854775807",
            "9223372036854775808",
        ];
        let _result: Vec<Natural> = values
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    #[test]
    fn test_invalid_naturals() -> Result<()> {
        let values = vec![
            "",
            "abc",
            "1.",
            "1.0",
            " 10",
            " -10",
            "- 10",
            "10%",
            "-9223372036854775809",
            "-9223372036854775808",
            "-2147483648",
            "-32768",
            "-128",
            "-1",
        ];
        let results: Vec<Result<Natural>> =
            values.into_iter().map(|item| item.try_into()).collect();

        for result in results {
            assert!(result.is_err())
        }

        Ok(())
    }
}
