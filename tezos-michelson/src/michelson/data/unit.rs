use crate::{
    micheline::{prim, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

use super::Data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit;

impl PrimType for Unit {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("Unit", &[11]);

impl From<Unit> for Data {
    fn from(value: Unit) -> Self {
        Self::Unit(value)
    }
}

impl TryFrom<Data> for Unit {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Unit(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}

impl From<()> for Unit {
    fn from(_: ()) -> Self {
        Unit
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

impl From<Unit> for Micheline {
    fn from(value: Unit) -> Self {
        prim(value.prim().name())
    }
}

impl From<Unit> for Michelson {
    fn from(value: Unit) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Unit {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for Unit {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }

        Ok(Unit)
    }
}

pub fn unit() -> Michelson {
    Unit.into()
}
