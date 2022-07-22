use tezos_core::types::encoded::{ImplicitAddress, ProtocolHash};

use super::{OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposals {
    pub source: ImplicitAddress,
    pub period: i32,
    pub proposals: Vec<ProtocolHash>,
}

impl Proposals {
    pub fn new(source: ImplicitAddress, period: i32, proposals: Vec<ProtocolHash>) -> Self {
        Self {
            source,
            period,
            proposals,
        }
    }
}

impl TraitOperationContent for Proposals {
    fn tag() -> OperationContentTag {
        OperationContentTag::Proposals
    }
}
