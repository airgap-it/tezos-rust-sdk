pub mod annotations;
pub mod data;
pub mod metadata;
pub mod types;

use annotations::Annotation;
use std::str::FromStr;

use self::{
    data::{instructions::Instruction, Data},
    types::Type,
};
use crate::{
    micheline::{literals::Literal, primitive_application::PrimitiveApplication, Micheline},
    Error, Result,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Michelson {
    Data(Data),
    Type(Type),
}

impl Michelson {
    pub fn prim_values() -> Vec<&'static Prim> {
        vec![Data::prim_values(), Type::prim_values()].concat()
    }

    pub fn is_michelson_data(&self) -> bool {
        if let Self::Data(_) = self {
            return true;
        }
        return false;
    }

    pub fn is_michelson_type(&self) -> bool {
        if let Self::Type(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_michelson_data(self) -> Option<Data> {
        if let Self::Data(value) = self {
            return Some(value);
        }
        None
    }

    pub fn into_michelson_type(self) -> Option<Type> {
        if let Self::Type(value) = self {
            return Some(value);
        }
        None
    }

    fn is_michelson_instruction(&self) -> bool {
        if let Self::Data(value) = self {
            return value.is_michelson_instruction();
        }
        return false;
    }

    fn is_michelson_elt(&self) -> bool {
        if let Self::Data(value) = self {
            return value.is_michelson_elt();
        }
        return false;
    }
}

impl From<Data> for Michelson {
    fn from(value: Data) -> Self {
        Self::Data(value)
    }
}

impl TryFrom<Michelson> for Data {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        if let Michelson::Data(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelson)
    }
}

impl TryFrom<Micheline> for Michelson {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        match value {
            Micheline::Literal(value) => Ok(value.into()),
            Micheline::PrimitiveApplication(value) => value.try_into(),
            Micheline::Sequence(value) => value.try_into(),
        }
    }
}

impl From<Literal> for Michelson {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Int(value) => value.into(),
            Literal::String(value) => value.into(),
            Literal::Bytes(value) => value.into(),
        }
    }
}

impl TryFrom<PrimitiveApplication> for Michelson {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if Data::is_valid_prim_name(value.prim()) {
            return Ok(Self::Data(value.try_into()?));
        }
        if Type::is_valid_prim_name(value.prim()) {
            return Ok(Self::Type(value.try_into()?));
        }

        Err(Error::InvalidPrimitiveApplication)
    }
}

impl TryFrom<crate::micheline::sequence::Sequence> for Michelson {
    type Error = Error;

    fn try_from(value: crate::micheline::sequence::Sequence) -> Result<Self> {
        value.into_values().try_into()
    }
}

impl TryFrom<Vec<Micheline>> for Michelson {
    type Error = Error;

    fn try_from(value: Vec<Micheline>) -> Result<Self> {
        if value.is_empty() {
            return Ok(data::sequence(vec![]).into());
        }

        let michelson_values = value
            .into_iter()
            .map(|value| value.try_into())
            .collect::<Result<Vec<Michelson>>>()?;

        if michelson_values
            .iter()
            .all(|value| value.is_michelson_instruction())
        {
            let instructions = michelson_values
                .into_iter()
                .map(|value| {
                    value
                        .into_michelson_data()
                        .unwrap()
                        .into_michelson_instruction()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            return Ok(data::instructions::sequence(instructions));
        }

        if michelson_values
            .iter()
            .all(|value| value.is_michelson_elt())
        {
            let elts = michelson_values
                .into_iter()
                .map(|value| {
                    value
                        .into_michelson_data()
                        .unwrap()
                        .into_michelson_elt()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            return Ok(data::map(elts));
        }

        if michelson_values
            .iter()
            .any(|value| value.is_michelson_elt())
        {
            return Err(Error::InvalidMicheline);
        }

        if michelson_values
            .iter()
            .all(|value| value.is_michelson_data())
        {
            let data_values = michelson_values
                .into_iter()
                .map(|value| value.into_michelson_data().unwrap())
                .collect::<Vec<_>>();
            return Ok(data::sequence(data_values));
        }

        Err(Error::InvalidMicheline)
    }
}

pub struct Prim {
    name: &'static str,
    tag: &'static [u8],
}

impl Prim {
    pub const fn new(name: &'static str, tag: &'static [u8]) -> Self {
        Self { name, tag }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn tag(&self) -> &'static [u8] {
        self.tag
    }

    pub fn from_data_name(name: &str) -> Result<&'static Self> {
        Data::prim_values()
            .into_iter()
            .find(|prim| prim.name() == name)
            .ok_or(Error::UnknownMichelsonPrimName)
    }

    pub fn from_data_tag(tag: &[u8]) -> Result<&'static Self> {
        Data::prim_values()
            .into_iter()
            .find(|prim| prim.tag() == tag)
            .ok_or(Error::UnknownMichelsonPrimTag)
    }

    pub fn from_instruction_name(name: &str) -> Result<&'static Self> {
        Instruction::prim_values()
            .into_iter()
            .find(|prim| prim.name() == name)
            .map(|prim| *prim)
            .ok_or(Error::UnknownMichelsonPrimName)
    }

    pub fn from_instruction_tag(tag: &[u8]) -> Result<&'static Self> {
        Instruction::prim_values()
            .into_iter()
            .find(|prim| prim.tag() == tag)
            .map(|prim| *prim)
            .ok_or(Error::UnknownMichelsonPrimTag)
    }

    pub fn from_type_name(name: &str) -> Result<&'static Self> {
        Type::prim_values()
            .into_iter()
            .find(|prim| prim.name() == name)
            .ok_or(Error::UnknownMichelsonPrimName)
    }

    pub fn from_type_tag(tag: &[u8]) -> Result<&'static Self> {
        Type::prim_values()
            .into_iter()
            .find(|prim| prim.tag() == tag)
            .ok_or(Error::UnknownMichelsonPrimTag)
    }

    pub fn from_comparable_type_name(name: &str) -> Result<&'static Self> {
        types::comparables::Type::prim_values()
            .into_iter()
            .find(|prim| prim.name() == name)
            .map(|prim| *prim)
            .ok_or(Error::UnknownMichelsonPrimName)
    }

    pub fn from_comparable_type_tag(tag: &[u8]) -> Result<&'static Self> {
        types::comparables::Type::prim_values()
            .into_iter()
            .find(|prim| prim.tag() == tag)
            .map(|prim| *prim)
            .ok_or(Error::UnknownMichelsonPrimTag)
    }
}

impl FromStr for &'static Prim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.try_into()
    }
}

impl TryFrom<&str> for &'static Prim {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Michelson::prim_values()
            .into_iter()
            .find(|prim| prim.name() == value)
            .ok_or(Error::UnknownMichelsonPrimName)
    }
}

impl TryFrom<&[u8]> for &'static Prim {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        Michelson::prim_values()
            .into_iter()
            .find(|prim| prim.tag == value)
            .ok_or(Error::UnknownMichelsonPrimTag)
    }
}

pub trait PrimType {
    fn prim_value() -> &'static Prim;
    fn prim(&self) -> &'static Prim {
        Self::prim_value()
    }
}
