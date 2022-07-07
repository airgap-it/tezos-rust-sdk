use {
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeedNonceRevelation {
    /// [OperationKind::SeedNonceRevelation]
    pub kind: OperationKind,
    /// integer ∈ [-2^31-1, 2^31]
    pub level: i32,
    /// /^([a-zA-Z0-9][a-zA-Z0-9])*$/
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
