use {
    crate::{
        models::balance_update::BalanceUpdate, models::error::RPCError,
        models::operation::kind::Kind, models::operation::operation_result::Status,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Delegation {
    /// [Kind::Delegation]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DelegationMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelegationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: DelegationOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalDelegationOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelegationOperationResult {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalDelegationOperationResult {
    /// [Kind::Delegation]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    pub result: DelegationOperationResult,
}
