use crate::{
    micheline::{prim_with_args, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, PartialEq)]
pub struct Some(Box<Data>);

impl Some {
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

impl PrimType for Some {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("Some", &[9]);

impl TryFrom<Data> for Some {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Some(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<Some> for Data {
    fn from(value: Some) -> Self {
        Self::Some(value)
    }
}

impl From<Some> for Micheline {
    fn from(value: Some) -> Self {
        prim_with_args(value.prim().name(), vec![value.to_value().into()])
    }
}

impl From<Some> for Michelson {
    fn from(value: Some) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Some {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for Some {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        let mut args = value.to_args().unwrap_or(vec![]);
        if args.is_empty() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(Some::new(args.remove(0).try_into()?))
    }
}

pub fn some(value: Data) -> Michelson {
    Some::new(value).into()
}
