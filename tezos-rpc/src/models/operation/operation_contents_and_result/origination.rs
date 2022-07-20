use {
    crate::{
        models::balance_update::BalanceUpdate, models::contract::ContractScript,
        models::operation::kind::OperationKind,
        models::operation::operation_result::operations::origination::OriginationOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Origination {
    /// [OperationKind::Origination]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub balance: Mutez,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    pub script: ContractScript,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OriginationMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OriginationMetadata {
    pub operation_result: OriginationOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
