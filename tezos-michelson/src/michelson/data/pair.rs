use super::Data;
use crate::{
    micheline::{prim_with_args, primitive_application::PrimitiveApplication, Micheline},
    michelson::{Michelson, Prim, PrimType},
    Error, Result,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Pair(Vec<Data>);

impl Pair {
    pub fn values(&self) -> &[Data] {
        return &self.0;
    }

    pub fn to_values(self) -> Vec<Data> {
        return self.0;
    }

    pub fn new(values: Vec<Data>) -> Result<Self> {
        if values.len() >= 2 {
            return Ok(Self(values));
        }
        Err(Error::InvalidVecValue)
    }
}

impl PrimType for Pair {
    fn prim_value() -> &'static Prim {
        &PRIM
    }
}

pub const PRIM: Prim = Prim::new("Pair", &[7]);

impl TryFrom<Vec<Data>> for Pair {
    type Error = Error;

    fn try_from(values: Vec<Data>) -> Result<Self> {
        Self::new(values)
    }
}

impl From<Pair> for Vec<Data> {
    fn from(value: Pair) -> Self {
        value.0
    }
}

impl From<Pair> for Data {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl TryFrom<Data> for Pair {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Pair(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonData)
    }
}

impl From<Pair> for Micheline {
    fn from(value: Pair) -> Self {
        prim_with_args(
            value.prim().name(),
            value
                .to_values()
                .into_iter()
                .map(|arg| arg.into())
                .collect(),
        )
    }
}

impl From<Pair> for Michelson {
    fn from(value: Pair) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Pair {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<PrimitiveApplication> for Pair {
    type Error = Error;

    fn try_from(value: PrimitiveApplication) -> Result<Self> {
        if value.prim() != PRIM.name() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        let args = value.to_args().unwrap_or(vec![]);
        if args.is_empty() {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(Pair::new(
            args.into_iter()
                .map(|value| value.try_into())
                .collect::<Result<Vec<Data>>>()?,
        )?)
    }
}

pub fn pair(first: Data, second: Data) -> Michelson {
    Pair::new(vec![first, second]).unwrap().into()
}
