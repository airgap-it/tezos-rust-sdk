use {
    crate::models::operation::kind::OperationKind,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ballot {
    /// [OperationKind::Ballot]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [-2^31-1, 2^31]
    pub period: i32,
    /// A protocol identifier (Base58Check-encoded)
    pub proposal: String,
    /// Ballot statement
    pub ballot: BallotStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BallotStatement {
    Nay,
    Yay,
    Pass,
}
