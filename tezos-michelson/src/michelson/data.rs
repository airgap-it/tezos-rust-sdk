pub mod instructions;
mod macros;
mod map;
mod sequence;

use self::macros::{make_all_data, make_data};
pub use crate::common::{bytes::Bytes, string::String};
pub use map::{map, Map};
pub use sequence::{sequence, Sequence};
pub use tezos_core::types::number::{integer::Integer as Int, natural::Natural as Nat};

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

    pub fn is_michelson_int(&self) -> bool {
        if let Self::Int(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_int(self) -> Option<Int> {
        if let Self::Int(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_nat(&self) -> bool {
        if let Self::Nat(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_nat(self) -> Option<Nat> {
        if let Self::Nat(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_string(&self) -> bool {
        if let Self::String(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_string(self) -> Option<String> {
        if let Self::String(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_bytes(&self) -> bool {
        if let Self::Bytes(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_bytes(self) -> Option<Bytes> {
        if let Self::Bytes(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_unit(&self) -> bool {
        if let Self::Unit(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_unit(self) -> Option<Unit> {
        if let Self::Unit(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_true(&self) -> bool {
        if let Self::True(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_true(self) -> Option<True> {
        if let Self::True(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_false(&self) -> bool {
        if let Self::False(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_false(self) -> Option<False> {
        if let Self::False(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_pair(&self) -> bool {
        if let Self::Int(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_pair(self) -> Option<Pair> {
        if let Self::Pair(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_left(&self) -> bool {
        if let Self::Left(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_left(self) -> Option<Left> {
        if let Self::Left(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_right(&self) -> bool {
        if let Self::Right(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_right(self) -> Option<Right> {
        if let Self::Right(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_some(&self) -> bool {
        if let Self::Some(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_some(self) -> Option<Some> {
        if let Self::Some(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_none(&self) -> bool {
        if let Self::Int(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_none(self) -> Option<None> {
        if let Self::None(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_sequence(&self) -> bool {
        if let Self::Sequence(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_sequence(self) -> Option<Sequence> {
        if let Self::Sequence(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_elt(&self) -> bool {
        if let Self::Elt(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_elt(self) -> Option<Elt> {
        if let Self::Elt(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_map(&self) -> bool {
        if let Self::Map(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_map(self) -> Option<Map> {
        if let Self::Map(value) = self {
            return Option::Some(value);
        }
        Option::None
    }

    pub fn is_michelson_instruction(&self) -> bool {
        if let Self::Instruction(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_instruction(self) -> Option<Instruction> {
        if let Self::Instruction(value) = self {
            return Option::Some(value);
        }
        Option::None
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

impl From<Int> for Data {
    fn from(value: Int) -> Self {
        Data::Int(value)
    }
}

impl From<i8> for Data {
    fn from(value: i8) -> Self {
        let integer: Int = value.into();
        integer.into()
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

impl From<Nat> for Data {
    fn from(value: Nat) -> Self {
        Data::Nat(value)
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

pub fn int<T>(value: T) -> Michelson
where
    T: std::convert::Into<Int>,
{
    let int: Int = value.into();
    int.into()
}

pub fn try_int<T, Error>(value: T) -> std::result::Result<Michelson, Error>
where
    T: std::convert::TryInto<Int, Error = Error>,
{
    let int: Int = value.try_into()?;
    Ok(int.into())
}

pub fn nat<T>(value: T) -> Michelson
where
    T: std::convert::Into<Nat>,
{
    let nat: Nat = value.into();
    nat.into()
}

pub fn try_nat<T, Error>(value: T) -> std::result::Result<Michelson, Error>
where
    T: std::convert::TryInto<Nat, Error = Error>,
{
    let nat: Nat = value.try_into()?;
    Ok(nat.into())
}

pub fn string<T>(value: T) -> Michelson
where
    T: std::convert::Into<String>,
{
    let string: String = value.into();
    string.into()
}

pub fn try_string<T, Error>(value: T) -> std::result::Result<Michelson, Error>
where
    T: std::convert::TryInto<String, Error = Error>,
{
    let string: String = value.try_into()?;
    Ok(string.into())
}

pub fn bytes<T>(value: T) -> Michelson
where
    T: std::convert::Into<Bytes>,
{
    let bytes: Bytes = value.into();
    bytes.into()
}

pub fn try_bytes<T, Error>(value: T) -> std::result::Result<Michelson, Error>
where
    T: std::convert::TryInto<Bytes, Error = Error>,
{
    let bytes: Bytes = value.try_into()?;
    Ok(bytes.into())
}
