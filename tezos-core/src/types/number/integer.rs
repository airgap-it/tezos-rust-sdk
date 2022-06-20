use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer as Int;
use num_traits::{Num, ToPrimitive};
use regex::Regex;
use std::{fmt::Debug, str::FromStr};

use crate::{
    internal::coder::{number::integer::IntegerBytesCoder, Decoder, Encoder},
    Error, Result,
};

use super::natural::Natural;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer(String);

impl Integer {
    pub fn from_string(value: String) -> Result<Self> {
        if Self::is_valid(&value) {
            return Ok(Integer(value));
        }
        Err(Error::InvalidIntegerString)
    }

    pub fn from_intenger<I: Int + ToString>(value: I) -> Self {
        Self::from_string(value.to_string()).unwrap()
    }

    pub fn to_integer<I: Int + FromStr>(&self) -> Result<I>
    where
        <I as FromStr>::Err: Debug,
    {
        Ok(self
            .0
            .parse::<I>()
            .map_err(|_error| Error::InvalidIntegerConversion)?)
    }

    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new(r"^-?[0-9]+$").unwrap();
        re.is_match(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        IntegerBytesCoder::encode(self)
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl ToPrimitive for Integer {
    fn to_i64(&self) -> Option<i64> {
        self.to_integer().ok()
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_integer().ok()
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_integer().ok()
    }

    fn to_u128(&self) -> Option<u128> {
        self.to_integer().ok()
    }
}

impl ToBigInt for Integer {
    fn to_bigint(&self) -> Option<BigInt> {
        BigInt::from_str_radix(&self.0, 10)
            .map(Some)
            .unwrap_or(None)
    }
}

impl From<i8> for Integer {
    fn from(value: i8) -> Self {
        Self::from_intenger(value)
    }
}

impl From<i16> for Integer {
    fn from(value: i16) -> Self {
        Self::from_intenger(value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Self::from_intenger(value)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self::from_intenger(value)
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        Self::from_intenger(value)
    }
}

impl From<BigInt> for Integer {
    fn from(value: BigInt) -> Self {
        Self::from_intenger(value)
    }
}

impl From<Natural> for Integer {
    fn from(value: Natural) -> Self {
        Integer(value.to_string())
    }
}

impl TryFrom<String> for Integer {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_string(value)
    }
}

impl TryFrom<&str> for Integer {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_string(value.to_string())
    }
}

impl TryFrom<&Vec<u8>> for Integer {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        IntegerBytesCoder::decode(value)
    }
}

impl TryFrom<&Integer> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Integer) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_integers() -> Result<()> {
        let integer_strings = vec![
            "-9223372036854775809",
            "-9223372036854775808",
            "-2147483648",
            "-32768",
            "-128",
            "-1",
            "0",
            "1",
            "127",
            "32767",
            "2147483647",
            "9223372036854775807",
            "9223372036854775808",
        ];
        let _integers: Vec<Integer> = integer_strings
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    #[test]
    fn test_invalid_integers() -> Result<()> {
        let integer_strings = vec!["", "abc", "1.", "1.0", " 10", " -10", "- 10", "10%"];
        let results: Vec<Result<Integer>> = integer_strings
            .into_iter()
            .map(|item| item.try_into())
            .collect();

        for result in results {
            assert!(result.is_err())
        }

        Ok(())
    }
}
