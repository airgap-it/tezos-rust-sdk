use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::transfer_ticket::TransferTicketOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{ContractAddress, ImplicitAddress},
        mutez::Mutez,
    },
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferTicket {
    /// [OperationKind::TransferTicket]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub ticket_contents: Micheline,
    pub ticket_ty: Micheline,
    pub ticket_ticketer: ContractAddress,
    pub ticket_amount: String,
    pub destination: ContractAddress,
    pub entrypoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TransferTicketMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferTicketMetadata {
    pub operation_result: TransferTicketOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
