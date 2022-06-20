use crate::{
    micheline::{prim, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct True;

impl PrimType for True {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("True", &[10]);

impl From<True> for Data {
    fn from(value: True) -> Self {
        Self::True(value)
    }
}

impl TryFrom<Data> for True {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::True(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}

impl From<True> for Micheline {
    fn from(value: True) -> Self {
        prim(value.prim().name())
    }
}

impl From<True> for Michelson {
    fn from(value: True) -> Self {
        Self::Data(value.into())
    }
}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
    }
}

impl TryFrom<Michelson> for True {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for True {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }

        Ok(True)
    }
}

pub fn r#true() -> Michelson {
    True.into()
}
