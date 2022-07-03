use {
    crate::{
        models::balance_update::BalanceUpdate,
        models::error::RPCError,
        models::operation::kind::Kind,
        models::operation::operation_result::{
            big_map_diff::BigMapDiff, lazy_storage_diff::LazyStorageDiff, Status,
        },
    },
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

const DEFAULT: &'static str = "default";
const ROOT: &'static str = "root";
const DO: &'static str = "do";
const SET_DELEGATE: &'static str = "set_delegate";
const REMOVE_DELEGATE: &'static str = "remove_delegate";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    /// [Kind::Transaction]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub amount: u64,
    /// Base58Check-encoded
    pub destination: String,
    pub parameters: Option<TransactionParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TransactionMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: TransactionOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalTransactionOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionOperationResult {
    pub status: Status,
    pub storage: Option<Micheline>, // FIXME: This should be Michelson
    pub big_map_diff: Option<BigMapDiff>,
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
    pub allocated_destination_contract: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<LazyStorageDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTransactionOperationResult {
    /// [Kind::Transaction]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Mutez
    pub amount: u64,
    /// Address (Base58Check-encoded)
    pub destination: String,
    pub result: TransactionOperationResult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionParameters {
    pub entrypoint: Entrypoint,
    pub value: Micheline, // FIXME: This should be Michelson
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Entrypoint {
    Default,
    Root,
    Do,
    SetDelegate,
    RemoveDelegate,
    Named(String),
}

impl From<&str> for Entrypoint {
    fn from(value: &str) -> Self {
        match value {
            DEFAULT => Entrypoint::Default,
            ROOT => Entrypoint::Root,
            DO => Entrypoint::Do,
            SET_DELEGATE => Entrypoint::SetDelegate,
            REMOVE_DELEGATE => Entrypoint::RemoveDelegate,
            _ => Entrypoint::Named(value.to_string()),
        }
    }
}

impl From<String> for Entrypoint {
    fn from(value: String) -> Self {
        match value.as_str() {
            DEFAULT => Entrypoint::Default,
            ROOT => Entrypoint::Root,
            DO => Entrypoint::Do,
            SET_DELEGATE => Entrypoint::SetDelegate,
            REMOVE_DELEGATE => Entrypoint::RemoveDelegate,
            _ => Entrypoint::Named(value),
        }
    }
}

impl<'a> Into<&'a str> for &'a Entrypoint {
    fn into(self) -> &'a str {
        match self {
            Entrypoint::Default => DEFAULT,
            Entrypoint::Root => ROOT,
            Entrypoint::Do => DO,
            Entrypoint::SetDelegate => SET_DELEGATE,
            Entrypoint::RemoveDelegate => REMOVE_DELEGATE,
            Entrypoint::Named(value) => value,
        }
    }
}

impl Into<String> for Entrypoint {
    fn into(self) -> String {
        match self {
            Entrypoint::Default => DEFAULT.to_string(),
            Entrypoint::Root => ROOT.to_string(),
            Entrypoint::Do => DO.to_string(),
            Entrypoint::SetDelegate => SET_DELEGATE.to_string(),
            Entrypoint::RemoveDelegate => REMOVE_DELEGATE.to_string(),
            Entrypoint::Named(value) => value,
        }
    }
}
