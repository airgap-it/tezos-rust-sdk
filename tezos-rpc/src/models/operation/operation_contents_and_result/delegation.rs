use tezos_core::types::{
    encoded::{Address, ImplicitAddress},
    mutez::Mutez,
};

use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::delegation::DelegationOperationResult,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Delegation {
    /// [OperationKind::Delegation]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
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
pub struct InternalDelegationOperationResult {
    /// [OperationKind::Delegation]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: Address,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    pub result: DelegationOperationResult,
}
