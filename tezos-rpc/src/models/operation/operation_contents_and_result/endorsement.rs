use {
    crate::models::balance_update::BalanceUpdate,
    crate::models::operation::kind::OperationKind,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Endorsement {
    /// [OperationKind::Endorsement]
    pub kind: OperationKind,
    /// integer ∈ [-2^31-1, 2^31]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EndorsementMetadata>,
    /// integer ∈ [0, 2^16-1]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u16>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
    /// Hash of a consensus value (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_payload_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndorsementMetadata {
    /// Public key hash (Base58Check-encoded)
    pub delegate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endorsement_power: Option<i32>,
    /// Legacy field (used in old protocols)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<u16>>,
}
