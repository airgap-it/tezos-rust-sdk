use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez};

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
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
    pub operation_result: DelegationOperationResult,
}
