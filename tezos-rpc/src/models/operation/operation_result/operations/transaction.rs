use {
    crate::{
        models::balance_update::BalanceUpdate,
        models::error::RpcError,
        models::operation::kind::OperationKind,
        models::operation::operation_contents_and_result::transaction::TransactionParameters,
        models::operation::operation_result::{
            big_map_diff::BigMapDiff, lazy_storage_diff::LazyStorageDiff, OperationResultStatus,
        },
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{Address, ContractAddress},
        mutez::Mutez,
    },
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<Vec<BigMapDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_contracts: Option<Vec<ContractAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_storage_size_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_destination_contract: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<Vec<LazyStorageDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionSuccessfulManagerOperationResult {
    /// [OperationKind::Transaction]
    pub kind: OperationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<BigMapDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_contracts: Option<Vec<ContractAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_storage_size_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_destination_contract: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<LazyStorageDiff>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InternalTransactionOperationResult {
    /// [OperationKind::Transaction]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: Address,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Mutez
    pub amount: Mutez,
    /// Address (Base58Check-encoded)
    pub destination: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransactionParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TransactionOperationResult>,
}
