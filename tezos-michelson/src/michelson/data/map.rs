use crate::{micheline::Micheline, michelson::Michelson, Error, Result};

use super::{Data, Elt};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Map(Vec<Elt>);

impl Map {
    pub fn values(&self) -> &[Elt] {
        &self.0
    }

    pub fn into_values(self) -> Vec<Elt> {
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

impl From<Map> for Micheline {
    fn from(value: Map) -> Self {
        value
            .into_values()
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<Micheline>>()
            .into()
    }
}

impl From<&Map> for Micheline {
    fn from(value: &Map) -> Self {
        value
            .values()
            .iter()
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

pub fn map<Output>(values: Vec<Elt>) -> Output
where
    Output: From<Map>,
{
    Map::new(values).into()
}
