use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::tx_rollup_commit::TxRollupCommitOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{CommitmentHash, ImplicitAddress, InboxHash, MessageResultHash, TxRollupId},
        mutez::Mutez,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupCommit {
    /// [OperationKind::TxRollupCommit]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub rollup: TxRollupId,
    pub commitment: TxRollupCommitment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TxRollupCommitMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupCommitMetadata {
    pub operation_result: TxRollupCommitOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupCommitment {
    /// integer ∈ [-2^31-1, 2^31],
    pub level: i32,
    pub messages: Vec<MessageResultHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessor: Option<CommitmentHash>,
    pub inbox_merkle_root: InboxHash,
}
