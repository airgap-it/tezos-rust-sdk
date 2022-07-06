use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use num_bigint::{BigUint, ToBigUint};
use num_traits::{Num, Unsigned};
use regex::Regex;

use crate::{
    internal::coder::{Decoder, Encoder, NaturalBytesCoder},
    types::mutez::Mutez,
    Error, Result,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nat(String);

impl Nat {
    pub fn from_string(value: String) -> Result<Self> {
        if Self::is_valid(&value) {
            return Ok(Nat(value));
        }
        Err(Error::InvalidUnsignedIntegerString)
    }

    pub fn from_integer<I: Unsigned + ToString>(value: I) -> Self {
        Self::from_string(value.to_string()).unwrap()
    }

    pub fn to_integer<I: Unsigned + FromStr>(&self) -> Result<I>
    where
        <I as FromStr>::Err: Debug,
    {
        Ok(self
            .0
            .parse::<I>()
            .map_err(|_error| Error::InvalidNaturalConversion)?)
    }

    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new(r"^[0-9]+$").unwrap();
        re.is_match(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        NaturalBytesCoder::encode(self)
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl Display for Nat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToBigUint for Nat {
    fn to_biguint(&self) -> Option<BigUint> {
        BigUint::from_str_radix(&self.0, 10)
            .map(Some)
            .unwrap_or(None)
    }
}

impl From<u8> for Nat {
    fn from(value: u8) -> Self {
        Self::from_integer(value)
    }
}

impl From<u16> for Nat {
    fn from(value: u16) -> Self {
        Self::from_integer(value)
    }
}

impl From<u32> for Nat {
    fn from(value: u32) -> Self {
        Self::from_integer(value)
    }
}

impl From<u64> for Nat {
    fn from(value: u64) -> Self {
        Self::from_integer(value)
    }
}

impl From<u128> for Nat {
    fn from(value: u128) -> Self {
        Self::from_integer(value)
    }
}

impl From<BigUint> for Nat {
    fn from(value: BigUint) -> Self {
        Self::from_integer(value)
    }
}

impl From<&Mutez> for Nat {
    fn from(mutez: &Mutez) -> Self {
        Self::from_integer(mutez.value())
    }
}

impl TryFrom<String> for Nat {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for Nat {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_string())
    }
}

impl TryFrom<&Vec<u8>> for Nat {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        NaturalBytesCoder::decode(value)
    }
}

impl TryFrom<&Nat> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Nat) -> Result<Self> {
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
        let _result: Vec<Nat> = values
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
        let results: Vec<Result<Nat>> = values.into_iter().map(|item| item.try_into()).collect();

        for result in results {
            assert!(result.is_err())
        }

        Ok(())
    }
}
