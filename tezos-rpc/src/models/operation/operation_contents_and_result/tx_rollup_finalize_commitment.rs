use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::tx_rollup_finalize_commitment::TxRollupFinalizeCommitmentOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{ImplicitAddress, TxRollupId},
        mutez::Mutez,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupFinalizeCommitment {
    /// [OperationKind::TxRollupFinalizeCommitment]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub rollup: TxRollupId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TxRollupFinalizeCommitmentMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupFinalizeCommitmentMetadata {
    pub operation_result: TxRollupFinalizeCommitmentOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
