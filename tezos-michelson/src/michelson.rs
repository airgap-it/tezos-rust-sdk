pub mod annotations;
pub mod data;
pub mod metadata;
pub mod types;

use annotations::Annotation;
use std::str::FromStr;
use tezos_core::internal::normalizer::Normalizer;

pub use self::{
    data::instructions::Primitive as InstructionPrimitive,
    data::Primitive as DataPrimitive,
    types::{ComparableTypePrimitive, Primitive as TypePrimitive},
};
use self::{
    data::{instructions::Instruction, Data},
    types::Type,
};
use crate::{
    internal::normalizer::MichelsonNormalizer,
    micheline::{literals::Literal, primitive_application::PrimitiveApplication, Micheline},
    Error, Result,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Michelson {
    Data(Data),
    Type(Type),
}

impl Michelson {
    pub fn pack(self, schema: Option<&Type>) -> Result<Vec<u8>> {
        let micheline: Micheline = self.into();
        let schema: Option<Micheline> = schema.map(|schema| schema.into());
        micheline.pack(schema.as_ref())
    }

    pub fn unpack(bytes: &[u8], schema: Option<&Type>) -> Result<Self> {
        let schema: Option<Micheline> = schema.map(|value| value.into());
        let micheline = Micheline::unpack(bytes, schema.as_ref())?;
        micheline.try_into()
    }

    pub fn normalized(self) -> Self {
        MichelsonNormalizer::normalize(self)
    }

    fn is_data(&self) -> bool {
        if let Self::Data(_) = self {
            return true;
        }
        return false;
    }

    fn is_instruction(&self) -> bool {
        if let Self::Data(Data::Instruction(_)) = self {
            return true;
        }
        return false;
    }

    fn is_elt(&self) -> bool {
        if let Self::Data(Data::Elt(_)) = self {
            return true;
        }
        return false;
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
            return Ok(data::sequence(vec![]));
        }

        let michelson_values = value
            .into_iter()
            .map(|value| value.try_into())
            .collect::<Result<Vec<Michelson>>>()?;

        if michelson_values.iter().all(|value| value.is_instruction()) {
            let instructions: Vec<Instruction> = michelson_values
                .into_iter()
                .map(|value| value.try_into().unwrap())
                .collect::<Vec<_>>();
            return Ok(data::instructions::sequence(instructions));
        }

        if michelson_values.iter().all(|value| value.is_elt()) {
            let elts: Vec<data::Elt> = michelson_values
                .into_iter()
                .map(|value| value.try_into().unwrap())
                .collect::<Vec<_>>();
            return Ok(data::map(elts));
        }

        if michelson_values.iter().any(|value| value.is_elt()) {
            return Err(Error::InvalidMicheline {
                description: format!("Invalid sequence of values: {:?}", michelson_values),
            });
        }

        if michelson_values.iter().all(|value| value.is_data()) {
            let data_values: Vec<Data> = michelson_values
                .into_iter()
                .map(|value| value.try_into().unwrap())
                .collect::<Vec<_>>();
            return Ok(data::sequence(data_values));
        }

        Err(Error::InvalidMicheline {
            description: format!("Invalid sequence of values: {:?}", michelson_values),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Data(DataPrimitive),
    Instruction(InstructionPrimitive),
    Type(TypePrimitive),
    ComparableType(ComparableTypePrimitive),
}

impl Primitive {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Data(primitive) => primitive.to_str(),
            Self::Instruction(primitive) => primitive.to_str(),
            Self::Type(primitive) => primitive.to_str(),
            Self::ComparableType(primitive) => primitive.to_str(),
        }
    }

    pub fn tag(&self) -> u8 {
        match self {
            Self::Data(primitive) => primitive.to_u8(),
            Self::Instruction(primitive) => primitive.to_u8(),
            Self::Type(primitive) => primitive.to_u8(),
            Self::ComparableType(primitive) => primitive.to_u8(),
        }
    }

    pub fn from_data_name(name: &str) -> Result<Self> {
        Ok(Self::Data(name.parse()?))
    }

    pub fn from_data_tag(tag: u8) -> Result<Self> {
        Ok(Self::Data(tag.try_into()?))
    }

    pub fn from_instruction_name(name: &str) -> Result<Self> {
        Ok(Self::Instruction(name.parse()?))
    }

    pub fn from_instruction_tag(tag: u8) -> Result<Self> {
        Ok(Self::Instruction(tag.try_into()?))
    }

    pub fn from_type_name(name: &str) -> Result<Self> {
        Ok(Self::Type(name.parse()?))
    }

    pub fn from_type_tag(tag: u8) -> Result<Self> {
        Ok(Self::Type(tag.try_into()?))
    }

    pub fn from_comparable_type_name(name: &str) -> Result<Self> {
        Ok(Self::ComparableType(name.parse()?))
    }

    pub fn from_comparable_type_tag(tag: u8) -> Result<Self> {
        Ok(Self::ComparableType(tag.try_into()?))
    }
}

impl FromStr for Primitive {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.try_into()
    }
}

impl TryFrom<&str> for Primitive {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let primitive = value.parse::<data::Primitive>();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::Data(primitive));
        }
        let primitive = value.parse::<data::instructions::Primitive>();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::Instruction(primitive));
        }
        let primitive = value.parse::<types::Primitive>();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::Type(primitive));
        }
        let primitive = value.parse::<types::ComparableTypePrimitive>();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::ComparableType(primitive));
        }
        Err(Error::InvalidStringValue)
    }
}

impl TryFrom<u8> for Primitive {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let primitive: Result<data::Primitive> = value.try_into();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::Data(primitive));
        }
        let primitive: Result<data::instructions::Primitive> = value.try_into();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::Instruction(primitive));
        }
        let primitive: Result<types::Primitive> = value.try_into();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::Type(primitive));
        }
        let primitive: Result<types::ComparableTypePrimitive> = value.try_into();
        if let Ok(primitive) = primitive {
            return Ok(Primitive::ComparableType(primitive));
        }
        Err(Error::InvalidBytes)
    }
}

impl From<Primitive> for String {
    fn from(value: Primitive) -> Self {
        value.name().into()
    }
}

pub trait PrimType {
    fn prim_value() -> Primitive;
    fn prim(&self) -> Primitive {
        Self::prim_value()
    }
}
