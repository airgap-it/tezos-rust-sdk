use tezos_core::{
    internal::types::BytesTag,
    types::encoded::{ImplicitAddress, ProtocolHash},
};

use super::{OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone)]
pub struct Ballot {
    source: ImplicitAddress,
    period: i32,
    proposal: ProtocolHash,
    ballot: Type,
}

impl Ballot {
    pub fn source(&self) -> &ImplicitAddress {
        &self.source
    }

    pub fn period(&self) -> i32 {
        self.period
    }

    pub fn proposal(&self) -> &ProtocolHash {
        &self.proposal
    }

    pub fn ballot(&self) -> Type {
        self.ballot
    }

    pub fn new(source: ImplicitAddress, period: i32, proposal: ProtocolHash, ballot: Type) -> Self {
        Self {
            source,
            period,
            proposal,
            ballot,
        }
    }
}

impl TraitOperationContent for Ballot {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::Ballot as u8]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Yay,
    Nay,
    Pass,
}

impl BytesTag for Type {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::Yay => &[0],
            Self::Nay => &[1],
            Self::Pass => &[2],
        }
    }
}
