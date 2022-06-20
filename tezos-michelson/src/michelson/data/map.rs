use crate::{micheline::Micheline, michelson::Michelson, Error, Result};

use super::{Data, Elt};

#[derive(Debug, Clone, PartialEq)]
pub struct Map(Vec<Elt>);

impl Map {
    pub fn values(&self) -> &[Elt] {
        &self.0
    }

    pub fn to_values(self) -> Vec<Elt> {
        self.0
    }

    pub fn new(values: Vec<Elt>) -> Self {
        Self(values)
    }
}

impl From<Vec<Elt>> for Map {
    fn from(values: Vec<Elt>) -> Self {
        Self::new(values)
    }
}

impl From<Map> for Vec<Elt> {
    fn from(value: Map) -> Self {
        value.0
    }
}

impl From<Map> for Data {
    fn from(value: Map) -> Self {
        Self::Map(value)
    }
}

impl TryFrom<Data> for Map {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self> {
        if let Data::Map(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidMichelsonData)
    }
}

impl From<Map> for Micheline {
    fn from(value: Map) -> Self {
        value
            .to_values()
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<Micheline>>()
            .into()
    }
}

impl From<Map> for Michelson {
    fn from(value: Map) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Map {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

pub fn map(values: Vec<Elt>) -> Michelson {
    Map::new(values).into()
}