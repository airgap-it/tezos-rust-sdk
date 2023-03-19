#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tezos_core::internal::normalizer::Normalizer;

use super::Micheline;
use crate::{internal::normalizer::MichelineNormalizer, Error, Result};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// `Micheline` primitive application as defined in [the documentation](https://tezos.gitlab.io/shell/micheline.html#bnf-grammar).
#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn args_count(&self) -> usize {
        self.args.as_ref().map(|args| args.len()).unwrap_or(0)
    }

    pub fn into_args(self) -> Option<Vec<Micheline>> {
        self.args
    }

    pub fn first_arg(&self) -> Option<&Micheline> {
        self.args.as_ref().map(|args| args.first()).flatten()
    }

    pub fn second_arg(&self) -> Option<&Micheline> {
        self.args.as_ref().map(|args| args.iter().nth(1)).flatten()
    }

    pub fn nth_arg(&self, n: usize) -> Option<&Micheline> {
        self.args.as_ref().map(|args| args.iter().nth(n)).flatten()
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

    pub fn with_args(mut self, args: Vec<Micheline>) -> Self {
        self.args = if !args.is_empty() { Some(args) } else { None };

        self
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

    pub fn try_with_mutated_args<F, Error>(
        mut self,
        mutator: F,
    ) -> core::result::Result<Self, Error>
    where
        F: FnOnce(Vec<Micheline>) -> core::result::Result<Vec<Micheline>, Error>,
    {
        if let Some(args) = self.args {
            self.args = Some(mutator(args)?)
        }
        Ok(self)
    }

    pub fn try_with_replaced_arg_at<F, Error>(
        mut self,
        index: usize,
        replacer: F,
    ) -> core::result::Result<Self, Error>
    where
        F: FnOnce(Micheline) -> core::result::Result<Micheline, Error>,
    {
        if let Some(args) = self.args.as_mut() {
            let element = args.remove(index);
            args.insert(index, replacer(element)?);
        }
        Ok(self)
    }

    pub fn with_annots(mut self, annots: Vec<String>) -> Self {
        self.annots = if !annots.is_empty() {
            Some(annots)
        } else {
            None
        };

        self
    }
}

impl TryFrom<Micheline> for PrimitiveApplication {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        if let Micheline::PrimitiveApplication(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidMicheline {
            description: format!("Cannot convert {:?} to a PrimitiveApplication", value),
        })
    }
}

impl From<PrimitiveApplication> for (String, Option<Vec<Micheline>>, Option<Vec<String>>) {
    fn from(value: PrimitiveApplication) -> Self {
        match value {
            PrimitiveApplication { prim, args, annots } => (prim, args, annots),
        }
    }
}
