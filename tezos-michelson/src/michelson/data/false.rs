use crate::{
    micheline::{prim, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct False;

impl PrimType for False {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("False", &[3]);

impl From<False> for Data {
    fn from(value: False) -> Self {
        Self::False(value)
    }
}

impl TryFrom<Data> for False {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::False(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}

impl From<False> for Micheline {
    fn from(value: False) -> Self {
        prim(value.prim().name())
    }
}

impl From<False> for Michelson {
    fn from(value: False) -> Self {
        Self::Data(value.into())
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        false
    }
}

impl TryFrom<Michelson> for False {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for False {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }

        Ok(False)
    }
}

pub fn r#false() -> Michelson {
    False.into()
}
