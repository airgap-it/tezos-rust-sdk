pub mod instructions;
mod macros;
mod map;
mod sequence;

use self::macros::{make_all_data, make_data};
pub use crate::common::{bytes::Bytes, string::String};
use crate::internal::normalizer::MichelsonNormalizer;
pub use map::{map, Map};
pub use sequence::{sequence, Sequence};
use tezos_core::internal::normalizer::Normalizer;
pub use tezos_core::types::number::{Int, Nat};

use alloc::vec::Vec;

make_all_data!(
    custom_cases: {
        Int(Int),
        Nat(Nat),
        String(String),
        Bytes(Bytes),
        Sequence(Sequence),
        Map(Map),
        Instruction(Instruction),
    },
    (Unit, unit, 11),
    (True, r#true, 10),
    (False, r#false, 3),
    (Pair, pair, 7, vec: (values: Data)),
    (Left, left, 5, boxed: (value: Data)),
    (Right, right, 8, boxed: (value: Data)),
    (Some, some, 9, boxed: (value: Data)),
    (None, none, 6),
    (Elt, elt, 4, boxed: (key: Data), boxed: (value: Data)),
);

impl Data {
    pub fn is_valid_prim_name(name: &str) -> bool {
        let primitive = name.parse::<Primitive>();
        if primitive.is_err() {
            return name.parse::<instructions::Primitive>().is_ok();
        }
        primitive.is_ok()
    }

    pub fn normalized(self) -> Self {
        MichelsonNormalizer::normalize(self)
    }
}

impl From<Literal> for Data {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Int(value) => Self::Int(value),
            Literal::String(value) => Self::String(value),
            Literal::Bytes(value) => Self::Bytes(value),
        }
    }
}

impl From<Int> for Michelson {
    fn from(value: Int) -> Self {
        Self::Data(value.into())
    }
}

impl From<i8> for Data {
    fn from(value: i8) -> Self {
        let integer: Int = value.into();
        integer.into()
    }
}

impl TryFrom<alloc::string::String> for Data {
    type Error = Error;

    fn try_from(value: alloc::string::String) -> Result<Self> {
        let value: String = value.try_into()?;
        Ok(value.into())
    }
}

impl TryFrom<Data> for i8 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Int(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<i32> for Data {
    fn from(value: i32) -> Self {
        let integer: Int = value.into();
        Data::Int(integer)
    }
}

impl TryFrom<Data> for i32 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Int(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        let integer: Int = value.into();
        Data::Int(integer)
    }
}

impl TryFrom<Data> for i64 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Int(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<i128> for Data {
    fn from(value: i128) -> Self {
        let integer: Int = value.into();
        Data::Int(integer)
    }
}

impl TryFrom<Data> for i128 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Int(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<Nat> for Michelson {
    fn from(value: Nat) -> Self {
        Self::Data(value.into())
    }
}

impl From<u8> for Data {
    fn from(value: u8) -> Self {
        let nat: Nat = value.into();
        nat.into()
    }
}

impl TryFrom<Data> for u8 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Nat(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<u16> for Data {
    fn from(value: u16) -> Self {
        let nat: Nat = value.into();
        nat.into()
    }
}

impl TryFrom<Data> for u16 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Nat(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<u32> for Data {
    fn from(value: u32) -> Self {
        let nat: Nat = value.into();
        nat.into()
    }
}

impl TryFrom<Data> for u32 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Nat(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<u64> for Data {
    fn from(value: u64) -> Self {
        let nat: Nat = value.into();
        nat.into()
    }
}

impl TryFrom<Data> for u64 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Nat(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<u128> for Data {
    fn from(value: u128) -> Self {
        let nat: Nat = value.into();
        nat.into()
    }
}

impl TryFrom<Data> for u128 {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Nat(value) = value {
            return Ok(value.to_integer()?);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        match value {
            true => Data::True(True),
            false => Data::False(False),
        }
    }
}

impl From<String> for Michelson {
    fn from(value: String) -> Self {
        Self::Data(value.into())
    }
}

impl From<Bytes> for Michelson {
    fn from(value: Bytes) -> Self {
        Self::Data(value.into())
    }
}

impl From<Unit> for () {
    fn from(_: Unit) -> Self {
        ()
    }
}

impl From<()> for Data {
    fn from(_: ()) -> Self {
        Unit.into()
    }
}

impl From<Primitive> for crate::michelson::Primitive {
    fn from(value: Primitive) -> Self {
        Self::Data(value)
    }
}

pub fn int<T, Output>(value: T) -> Output
where
    T: core::convert::Into<Int>,
    Output: From<Int>,
{
    let value: Int = value.into();
    value.into()
}

pub fn try_int<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<Int, Error = Error>,
    Output: From<Int>,
{
    let value: Int = value.try_into()?;
    Ok(value.into())
}

pub fn nat<T, Output>(value: T) -> Output
where
    T: core::convert::Into<Nat>,
    Output: From<Nat>,
{
    let value: Nat = value.into();
    value.into()
}

pub fn try_nat<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<Nat, Error = Error>,
    Output: From<Nat>,
{
    let value: Nat = value.try_into()?;
    Ok(value.into())
}

pub fn string<T, Output>(value: T) -> Output
where
    T: core::convert::Into<String>,
    Output: From<String>,
{
    let value: String = value.into();
    value.into()
}

pub fn try_string<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<String, Error = Error>,
    Output: From<String>,
{
    let value: String = value.try_into()?;
    Ok(value.into())
}

pub fn bytes<T, Output>(value: T) -> Output
where
    T: core::convert::Into<Bytes>,
    Output: From<Bytes>,
{
    let value: Bytes = value.into();
    value.into()
}

pub fn try_bytes<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<Bytes, Error = Error>,
    Output: From<Bytes>,
{
    let value: Bytes = value.try_into()?;
    Ok(value.into())
}

impl Pair {
    pub fn flatten(self) -> Self {
        let mut new_values = Vec::<Data>::new();
        for value in self.values {
            if let Data::Pair(pair) = value {
                new_values.append(&mut pair.flatten().values);
            } else {
                new_values.push(value)
            }
        }

        Self { values: new_values }
    }
}
