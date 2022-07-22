use {
    crate::models::operation::kind::OperationKind,
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::{ImplicitAddress, ProtocolHash},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ballot {
    /// [OperationKind::Ballot]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    /// integer ∈ [-2^31-1, 2^31]
    pub period: i32,
    /// A protocol identifier (Base58Check-encoded)
    pub proposal: ProtocolHash,
    /// Ballot statement
    pub ballot: BallotStatement,
}

impl From<tezos_operation::operations::Ballot> for Ballot {
    fn from(value: tezos_operation::operations::Ballot) -> Self {
        Self {
            kind: OperationKind::Ballot,
            source: value.source,
            period: value.period,
            proposal: value.proposal,
            ballot: value.ballot.into(),
        }
    }
}

impl From<Ballot> for tezos_operation::operations::Ballot {
    fn from(value: Ballot) -> Self {
        Self {
            source: value.source,
            period: value.period,
            proposal: value.proposal,
            ballot: value.ballot.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BallotStatement {
    Nay,
    Yay,
    Pass,
}

impl From<tezos_operation::operations::BallotType> for BallotStatement {
    fn from(value: tezos_operation::operations::BallotType) -> Self {
        match value {
            tezos_operation::operations::BallotType::Yay => Self::Yay,
            tezos_operation::operations::BallotType::Nay => Self::Nay,
            tezos_operation::operations::BallotType::Pass => Self::Pass,
        }
    }
}

impl From<BallotStatement> for tezos_operation::operations::BallotType {
    fn from(value: BallotStatement) -> Self {
        match value {
            BallotStatement::Nay => Self::Nay,
            BallotStatement::Yay => Self::Yay,
            BallotStatement::Pass => Self::Pass,
        }
    }
}
