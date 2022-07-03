use crate::models::{
    error::RPCError, operation::operation_result::lazy_storage_diff::LazyStorageDiff,
};

use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::Kind,
        models::operation::operation_result::Status,
    },
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Origination {
    /// [Kind::Origination]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub balance: u64,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    pub script: OriginationScript,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OriginationMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OriginationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: OriginationOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalOriginationOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginationScript {
    pub code: Vec<Micheline>,
    pub storage: Micheline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OriginationOperationResult {
    pub status: Status,
    pub big_map_diff: Option<String>, // FIXME: Add big_map_diff struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_contracts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_storage_size_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<LazyStorageDiff>, // FIXME: Add lazy_storage_diff struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalOriginationOperationResult {
    /// [Kind::Origination]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Mutez
    pub balance: u64,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<OriginationScript>,
    pub result: OriginationOperationResult,
}
