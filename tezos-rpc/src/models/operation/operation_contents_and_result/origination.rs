use {
    crate::{
        models::balance_update::BalanceUpdate, models::contract::ContractScript,
        models::operation::kind::OperationKind,
        models::operation::operation_result::operations::origination::OriginationOperationResult,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Origination {
    /// [OperationKind::Origination]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub balance: u64,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    pub script: ContractScript,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OriginationMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OriginationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: OriginationOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalOriginationOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalOriginationOperationResult {
    /// [OperationKind::Origination]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Mutez
    pub balance: u64,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ContractScript>,
    pub result: OriginationOperationResult,
}
