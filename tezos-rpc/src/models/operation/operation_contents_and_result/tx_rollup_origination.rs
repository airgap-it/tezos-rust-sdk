use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::tx_rollup_origination::TxRollupOriginationOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupOrigination {
    /// [OperationKind::TxRollupOrigination]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub tx_rollup_origination: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TxRollupOriginationMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupOriginationMetadata {
    pub operation_result: TxRollupOriginationOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
