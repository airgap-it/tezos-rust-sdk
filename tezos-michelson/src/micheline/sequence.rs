#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tezos_core::internal::{coder::Encoder, normalizer::Normalizer};

use super::Micheline;
use crate::{
    internal::{
        coder::micheline_bytes_coder::MichelineBytesCoder, normalizer::MichelineNormalizer,
    },
    Error, Result,
};
use alloc::format;
use alloc::vec::Vec;

/// `Micheline` sequence types as defined in [the documentation](https://tezos.gitlab.io/shell/micheline.html#bnf-grammar).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        MichelineBytesCoder::encode(self)
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
        Err(Error::InvalidMicheline {
            description: format!("Cannot convert {:?} to a Sequence", value),
        })
    }
}
