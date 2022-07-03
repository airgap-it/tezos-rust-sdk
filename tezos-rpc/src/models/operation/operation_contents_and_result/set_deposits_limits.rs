use {
    crate::models::{
        balance_update::BalanceUpdate, error::RPCError, operation::kind::Kind,
        operation::operation_result::Status,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositsLimits {
    /// [Kind::SetDepositsLimits]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SetDepositsLimitsMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositsLimitsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: SetDepositsLimitsOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalSetDepositsLimitsOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositsLimitsOperationResult {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalSetDepositsLimitsOperationResult {
    /// [Kind::SetDepositsLimits]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub result: SetDepositsLimitsOperationResult,
}
