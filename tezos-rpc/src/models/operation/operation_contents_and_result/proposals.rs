use {
    crate::models::operation::kind::Kind,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proposals {
    /// [Kind::Proposals]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [-2^31-1, 2^31]
    pub period: i32,
    /// A vector of protocol identifiers (Base58Check-encoded)
    pub proposals: Vec<String>,
}
