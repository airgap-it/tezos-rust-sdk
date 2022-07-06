use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::set_deposits_limit::SetDepositsLimitOperationResult,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositsLimit {
    /// [OperationKind::SetDepositsLimit]
    pub kind: OperationKind,
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
    pub operation_result: SetDepositsLimitOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalSetDepositsLimitOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalSetDepositsLimitOperationResult {
    /// [OperationKind::SetDepositsLimit]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    pub result: SetDepositsLimitOperationResult,
}
