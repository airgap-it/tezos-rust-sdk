use crate::{
    micheline::{prim_with_args, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, PartialEq)]
pub struct Left(Box<Data>);

impl Left {
    pub fn value(&self) -> &Data {
        &self.0
    }

    pub fn to_value(self) -> Data {
        *self.0
    }

    pub fn new(value: Data) -> Self {
        Self(Box::new(value))
    }
}

impl PrimType for Left {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("Left", &[5]);

impl TryFrom<Data> for Left {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Left(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}

impl From<Left> for Data {
    fn from(value: Left) -> Self {
        Self::Left(value)
    }
}

impl From<Left> for Micheline {
    fn from(value: Left) -> Self {
        prim_with_args(value.prim().name(), vec![value.to_value().into()])
    }
}

impl From<Left> for Michelson {
    fn from(value: Left) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Left {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for Left {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        let mut args = value.to_args().unwrap_or(vec![]);
        if args.is_empty() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(Left::new(args.remove(0).try_into()?))
    }
}

pub fn left(value: Data) -> Michelson {
    Left::new(value).into()
}
