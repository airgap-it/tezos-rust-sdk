use {
    crate::models::{
        operation::kind::OperationKind, operation::operation_result::OperationResultStatus,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::Address,
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EventOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InternalEventOperationResult {
    /// [OperationKind::Event]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: Address,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Micheline,
    /// Tag
    pub tag: String,
    /// Payload
    pub payload: Micheline,
    /// Result
    pub result: EventOperationResult,
}
