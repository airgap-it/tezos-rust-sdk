use crate::{
    micheline::{prim_with_args, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, PartialEq)]
pub struct Right(Box<Data>);

impl Right {
    pub fn value(&self) -> &Data {
        &self.0
    }

    pub fn to_value(self) -> Data {
        *self.0
    }

    pub fn new(value: Data) -> Self {
        Self(Box::new(value))
    }

    pub fn prim() -> &'static Prim {
        &PRIM
    }
}

impl PrimType for Right {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("Right", &[8]);

impl TryFrom<Data> for Right {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Right(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<Right> for Data {
    fn from(value: Right) -> Self {
        Self::Right(value)
    }
}

impl From<Right> for Micheline {
    fn from(value: Right) -> Self {
        prim_with_args(value.prim().name(), vec![value.to_value().into()])
    }
}

impl From<Right> for Michelson {
    fn from(value: Right) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Right {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for Right {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        let mut args = value.to_args().unwrap_or(vec![]);
        if args.is_empty() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(Right::new(args.remove(0).try_into()?))
    }
}

pub fn right(value: Data) -> Michelson {
    Right::new(value).into()
}
