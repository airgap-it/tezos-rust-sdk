use super::Data;
use crate::{
    micheline::{prim_with_args, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Elt(Box<(Data, Data)>);

impl Elt {
    pub fn key(&self) -> &Data {
        return &self.0 .0;
    }

    pub fn value(&self) -> &Data {
        return &self.0 .1;
    }

    pub fn to_args(self) -> Vec<Data> {
        vec![self.0 .0, self.0 .1]
    }

    pub fn new(key: Data, value: Data) -> Self {
        Self(Box::new((key, value)))
    }
}

impl PrimType for Elt {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("Elt", &[4]);

impl From<(Data, Data)> for Elt {
    fn from(value: (Data, Data)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Elt> for (Data, Data) {
    fn from(value: Elt) -> Self {
        *value.0
    }
}

impl From<Elt> for Data {
    fn from(value: Elt) -> Self {
        Self::Elt(value)
    }
}

impl TryFrom<Data> for Elt {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Elt(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<Elt> for Micheline {
    fn from(value: Elt) -> Self {
        prim_with_args(
            value.prim().name(),
            value.to_args().into_iter().map(|arg| arg.into()).collect(),
        )
    }
}

impl From<Elt> for Michelson {
    fn from(value: Elt) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Elt {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for Elt {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        let mut args = value.to_args().unwrap_or(vec![]);
        if args.len() < 2 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(Elt::new(
            args.remove(0).try_into()?,
            args.remove(0).try_into()?,
        ))
    }
}

pub fn elt(key: Data, value: Data) -> Michelson {
    Elt::new(key, value).into()
}
