use tezos_core::types::encoded::{ImplicitAddress, ProtocolHash};

use super::{OperationContentTag, TraitOperationContent};

pub struct Proposals {
    source: ImplicitAddress,
    period: i32,
    proposals: Vec<ProtocolHash>,
}

impl Proposals {
    pub fn source(&self) -> &ImplicitAddress {
        &self.source
    }

    pub fn period(&self) -> i32 {
        self.period
    }

    pub fn proposals(&self) -> &[ProtocolHash] {
        &self.proposals
    }

    pub fn new(source: ImplicitAddress, period: i32, proposals: Vec<ProtocolHash>) -> Self {
        Self {
            source,
            period,
            proposals,
        }
    }
}

impl TraitOperationContent for Proposals {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::Proposals as u8]
    }
}
