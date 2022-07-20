use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::tx_rollup_rejection::TxRollupRejectionOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{
            ContextHash, ImplicitAddress, InboxHash, MessageResultHash, MessageResultListHash,
            ScriptExprHash, TxRollupId, TxRollupL2Address, WithdrawListHash,
        },
        mutez::Mutez,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupRejection {
    /// [OperationKind::TxRollupRejection]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub rollup: TxRollupId,
    pub message: TxRollupRejectionMessage,
    pub message_position: String,
    pub message_path: Vec<InboxHash>,
    pub message_result_hash: MessageResultHash,
    pub message_result_path: Vec<MessageResultListHash>,
    pub previous_message_result: TxRollupRejectionMessageResult,
    pub previous_message_result_path: Vec<MessageResultListHash>,
    pub proof: TxRollupRejectionProof,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TxRollupRejectionMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupRejectionMetadata {
    pub operation_result: TxRollupRejectionOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TxRollupRejectionMessage {
    Batch(TxRollupRejectionBatchMessage),
    Deposit(TxRollupRejectionDepositMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupRejectionBatchMessage {
    pub batch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupRejectionDepositMessage {
    pub sender: ImplicitAddress,
    pub destination: TxRollupL2Address,
    pub ticket_hash: ScriptExprHash,
    pub amount: Mutez,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupRejectionMessageResult {
    pub context_hash: ContextHash,
    pub withdraw_list_hash: WithdrawListHash,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupRejectionProof {
    pub version: i16,
    pub before: ValueOrNodeContext,
    pub after: ValueOrNodeContext,
    pub state: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueContext {
    pub value: ContextHash,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeContext {
    pub node: ContextHash,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ValueOrNodeContext {
    Value(ValueContext),
    Node(NodeContext),
}
