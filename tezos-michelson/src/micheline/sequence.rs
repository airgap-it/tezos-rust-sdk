use serde::{Deserialize, Serialize};
use tezos_core::internal::normalizer::Normalizer;

use super::Micheline;
use crate::{internal::normalizer::MichelineNormalizer, Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence(Vec<Micheline>);

impl Sequence {
    pub fn values(&self) -> &[Micheline] {
        &self.0
    }

    pub fn into_values(self) -> Vec<Micheline> {
        self.0
    }

    pub fn normalized(self) -> Self {
        MichelineNormalizer::normalize(self)
    }
}

impl From<Vec<Micheline>> for Sequence {
    fn from(values: Vec<Micheline>) -> Self {
        Self(values)
    }
}

impl TryFrom<Micheline> for Sequence {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        if let Micheline::Sequence(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMicheline)
    }
}
