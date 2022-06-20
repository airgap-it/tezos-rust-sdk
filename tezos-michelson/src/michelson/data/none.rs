use crate::{
    micheline::{prim, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct None;

impl PrimType for None {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("None", &[6]);

impl From<None> for Data {
    fn from(value: None) -> Self {
        Self::None(value)
    }
}

impl TryFrom<Data> for None {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::None(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}

impl From<None> for Micheline {
    fn from(value: None) -> Self {
        prim(value.prim().name())
    }
}

impl From<None> for Michelson {
    fn from(value: None) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for None {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for None {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }

        Ok(None)
    }
}

pub fn none() -> Michelson {
    None.into()
}
