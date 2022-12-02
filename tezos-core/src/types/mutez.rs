//! Tezos Mutez type.

use std::{fmt::Debug, str::FromStr};

use derive_more;
use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display, Div,
    DivAssign, Mul, MulAssign, Not, Octal, Product, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    Sub, SubAssign, Sum,
};
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::internal::coder::{ConsumingDecoder, Decoder, Encoder, MutezBytesCoder};
use crate::internal::consumable_list::ConsumableList;
use crate::{Error, Result};

use super::number::Nat;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

/// Tezos Mutez type. It can be encoded into and initialized from bytes and many other number
/// representations.
///
/// # Example
///
/// ```
/// use tezos_core::types::mutez::Mutez;
/// let amount_1: Mutez = "24".try_into().expect("valid number string can be converted to Mutez");
/// let amount_2: Mutez = 42u8.into();
/// ```
///
/// Internally the number is represented with an [i64], but negative values are invalid.
#[derive(
    Add,
    AddAssign,
    PartialEq,
    PartialOrd,
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
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String")
)]
pub struct Mutez(#[cfg_attr(feature = "serde", serde(serialize_with = "i64_to_string"))] i64);

impl Mutez {
    pub fn is_valid(value: &str) -> bool {
        REGEX.is_match(value)
    }

    pub(super) fn value(&self) -> u64 {
        self.0.to_u64().unwrap()
    }
    /// Encodes the [Mutez] value to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        MutezBytesCoder::encode(self)
    }
    /// Creates the [Mutez] value from bytes that were previously generated using [Mutez::to_bytes].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        MutezBytesCoder::decode(bytes)
    }

    pub fn from_consumable_bytes<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Self> {
        MutezBytesCoder::decode_consuming(bytes)
    }
}

#[cfg(feature = "serde")]
fn i64_to_string<S>(value: &i64, s: S) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&value.to_string())
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

impl TryFrom<BigUint> for Mutez {
    type Error = Error;

    fn try_from(value: BigUint) -> Result<Self> {
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

impl Default for Mutez {
    fn default() -> Self {
        Self(0)
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

    #[test]
    fn test_cmp() -> Result<()> {
        let v1: Mutez = 1u8.into();
        let v2: Mutez = 2u8.into();

        assert!(v1 < v2);

        Ok(())
    }
}
