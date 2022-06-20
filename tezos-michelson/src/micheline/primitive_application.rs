use serde::{Deserialize, Serialize};
use tezos_core::internal::normalizer::Normalizer;

use super::Micheline;
use crate::{internal::normalizer::MichelineNormalizer, Error, Result};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrimitiveApplication {
    prim: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    args: Option<Vec<Micheline>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    annots: Option<Vec<String>>,
}

impl PrimitiveApplication {
    pub fn prim(&self) -> &str {
        &self.prim
    }

    pub fn args(&self) -> &Option<Vec<Micheline>> {
        &self.args
    }

    pub fn to_args(self) -> Option<Vec<Micheline>> {
        self.args
    }

    pub fn annots(&self) -> &Option<Vec<String>> {
        &self.annots
    }

    pub fn new(prim: String, args: Option<Vec<Micheline>>, annots: Option<Vec<String>>) -> Self {
        let mut args = args;
        if let Some(value) = &args {
            if value.is_empty() {
                args = None;
            }
        }
        let mut annots = annots;
        if let Some(value) = &annots {
            if value.is_empty() {
                annots = None;
            }
        }
        PrimitiveApplication { prim, args, annots }
    }

    pub fn normalized(self) -> Self {
        MichelineNormalizer::normalize(self)
    }

    pub fn with_mutated_args<F>(mut self, mutator: F) -> Self
    where
        F: FnOnce(Vec<Micheline>) -> Vec<Micheline>,
    {
        if let Some(args) = self.args {
            self.args = Some(mutator(args))
        }
        self
    }
}

impl TryFrom<Micheline> for PrimitiveApplication {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        if let Micheline::PrimitiveApplication(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidMicheline)
    }
}

impl From<PrimitiveApplication> for (String, Option<Vec<Micheline>>, Option<Vec<String>>) {
    fn from(value: PrimitiveApplication) -> Self {
        match value {
            PrimitiveApplication { prim, args, annots } => (prim, args, annots),
        }
    }
}
