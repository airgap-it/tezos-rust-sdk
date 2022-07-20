use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::tx_rollup_dispatch_tickets::TxRollupDispatchTicketsOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{
            ContextHash, ContractAddress, ImplicitAddress, MessageResultListHash, TxRollupId,
        },
        mutez::Mutez,
    },
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupDispatchTickets {
    /// [OperationKind::TxRollupDispatchTickets]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub tx_rollup: TxRollupId,
    pub level: i32,
    pub context_hash: ContextHash,
    pub message_index: i32,
    pub message_result_path: Vec<MessageResultListHash>,
    pub tickets_info: Vec<TicketInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TxRollupDispatchTicketsMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRollupDispatchTicketsMetadata {
    pub operation_result: TxRollupDispatchTicketsOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TicketInfo {
    pub contents: Micheline,
    pub ty: Micheline,
    pub ticketer: ContractAddress,
    pub amount: Mutez,
    pub claimer: ContractAddress,
}
