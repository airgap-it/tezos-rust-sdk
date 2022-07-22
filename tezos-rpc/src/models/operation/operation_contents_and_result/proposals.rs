use tezos_core::types::encoded::{ImplicitAddress, ProtocolHash};

use {
    crate::models::operation::kind::OperationKind,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proposals {
    /// [OperationKind::Proposals]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    /// integer ∈ [-2^31-1, 2^31]
    pub period: i32,
    /// A vector of protocol identifiers (Base58Check-encoded)
    pub proposals: Vec<ProtocolHash>,
}

impl From<tezos_operation::operations::Proposals> for Proposals {
    fn from(value: tezos_operation::operations::Proposals) -> Self {
        Self {
            kind: OperationKind::Proposals,
            source: value.source,
            period: value.period,
            proposals: value.proposals,
        }
    }
}

impl From<Proposals> for tezos_operation::operations::Proposals {
    fn from(value: Proposals) -> Self {
        Self {
            source: value.source,
            period: value.period,
            proposals: value.proposals,
        }
    }
}
