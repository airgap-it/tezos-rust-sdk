use super::Data;
use crate::{
    micheline::{self, Micheline},
    michelson::Michelson,
    Error, Result,
};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence(Vec<Data>);

impl Sequence {
    pub fn values(&self) -> &[Data] {
        return &self.0;
    }

    pub fn into_values(self) -> Vec<Data> {
        return self.0;
    }

    pub fn new(values: Vec<Data>) -> Self {
        Self(values)
    }
}

impl From<Vec<Data>> for Sequence {
    fn from(values: Vec<Data>) -> Self {
        Self::new(values)
    }
}

impl From<Vec<Data>> for Michelson {
    fn from(values: Vec<Data>) -> Self {
        let value: Sequence = values.into();
        value.into()
    }
}

impl From<Sequence> for Vec<Data> {
    fn from(value: Sequence) -> Self {
        value.0
    }
}

impl From<Sequence> for Micheline {
    fn from(value: Sequence) -> Self {
        value
            .into_values()
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<Micheline>>()
            .into()
    }
}

impl From<&Sequence> for Micheline {
    fn from(value: &Sequence) -> Self {
        value
            .values()
            .iter()
            .map(|value| value.into())
            .collect::<Vec<Micheline>>()
            .into()
    }
}

impl From<Sequence> for Michelson {
    fn from(value: Sequence) -> Self {
        Self::Data(value.into())
    }
}

impl TryFrom<Michelson> for Sequence {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let data: Data = value.try_into()?;
        data.try_into()
    }
}

impl TryFrom<Micheline> for Sequence {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        let sequence: micheline::sequence::Sequence = value.try_into()?;

        sequence.try_into()
    }
}

impl TryFrom<micheline::sequence::Sequence> for Sequence {
    type Error = Error;

    fn try_from(value: micheline::sequence::Sequence) -> Result<Self> {
        Ok(value
            .into_values()
            .into_iter()
            .map(|value| value.try_into())
            .collect::<Result<Vec<Data>>>()?
            .into())
    }
}

pub fn sequence<Output>(values: Vec<Data>) -> Output
where
    Output: From<Sequence>,
{
    Sequence::new(values).into()
}
