use {
    crate::{
        models::balance_update::BalanceUpdate,
        models::operation::kind::OperationKind,
        models::operation::operation_result::{
            operations::transaction::TransactionOperationResult
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
    /// [OperationKind::Transaction]
    pub kind: OperationKind,
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
pub struct InternalTransactionOperationResult {
    /// [OperationKind::Transaction]
    pub kind: OperationKind,
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
