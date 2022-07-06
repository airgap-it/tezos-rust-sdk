use std::{fmt::Debug, str::FromStr};

use derive_more;
use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display, Div,
    DivAssign, Mul, MulAssign, Not, Octal, Product, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    Sub, SubAssign, Sum,
};
use num_traits::ToPrimitive;
use regex::Regex;

use crate::internal::coder::{Decoder, Encoder, MutezBytesCoder};
use crate::{Error, Result};

use super::number::Nat;

#[derive(
    Add,
    AddAssign,
    PartialEq,
    Debug,
    Eq,
    Clone,
    Copy,
    Display,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Not,
    Octal,
    Product,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Sub,
    SubAssign,
    Sum,
)]
#[div(forward)]
#[div_assign(forward)]
#[mul(forward)]
#[mul_assign(forward)]
#[rem(forward)]
#[rem_assign(forward)]
pub struct Mutez(i64);

impl Mutez {
    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new(r"^[0-9]+$").unwrap();
        re.is_match(value)
    }

    pub(super) fn value(&self) -> u64 {
        self.0.to_u64().unwrap()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        MutezBytesCoder::encode(self)
    }
}

impl FromStr for Mutez {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if Self::is_valid(s) {
            return Ok(Self(s.parse::<i64>()?));
        }
        Err(Error::InvalidUnsignedIntegerString)
    }
}

impl From<u8> for Mutez {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<u16> for Mutez {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u32> for Mutez {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl TryFrom<u64> for Mutez {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self> {
        Ok(Self(value.try_into()?))
    }
}

impl From<Mutez> for String {
    fn from(mutez: Mutez) -> Self {
        mutez.0.to_string()
    }
}

impl ToPrimitive for Mutez {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }
}

impl TryFrom<Mutez> for u8 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_u8().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for i8 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_i8().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for u16 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_u16().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for i16 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_i16().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for u32 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_u32().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for i32 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_i32().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for u64 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_u64().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for i64 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_i64().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for u128 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_u128().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for i128 {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_i128().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for usize {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_usize().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<Mutez> for isize {
    type Error = Error;

    fn try_from(value: Mutez) -> Result<Self> {
        value.to_isize().ok_or(Error::InvalidConversion)
    }
}

impl TryFrom<String> for Mutez {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self::from_str(&value)?)
    }
}

impl TryFrom<&str> for Mutez {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Self::from_str(value)?)
    }
}

impl TryFrom<&Nat> for Mutez {
    type Error = Error;

    fn try_from(value: &Nat) -> Result<Self> {
        value.to_string().try_into()
    }
}

impl TryFrom<&Vec<u8>> for Mutez {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        MutezBytesCoder::decode(value)
    }
}

impl TryFrom<&Mutez> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Mutez) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_1() -> Result<()> {
        let v1: Mutez = 1u8.into();
        let v2: Mutez = "2".try_into()?;

        assert_eq!(v1 + v2, 3u32.into());

        Ok(())
    }

    #[test]
    fn test_add_2() -> Result<()> {
        let v1: Mutez = 1u8.into();
        let v2: Mutez = 2u8.into();

        assert_eq!(v1 + v2, 3u32.into());

        Ok(())
    }
}
