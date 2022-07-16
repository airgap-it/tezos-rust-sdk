use tezos_core::types::{
    encoded::{Address, ImplicitAddress},
    mutez::Mutez,
};

use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::transaction::TransactionOperationResult,
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
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub amount: Mutez,
    /// Base58Check-encoded
    pub destination: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransactionParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TransactionMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionMetadata {
    pub operation_result: TransactionOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
    #[serde(default)]
    pub internal_operation_results: Vec<InternalTransactionOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionParameters {
    pub entrypoint: Entrypoint,
    pub value: Micheline,
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

#[cfg(test)]
mod tests {
    use {super::*, crate::error::Error};

    #[tokio::test]
    async fn test_transaction_deserialization() -> Result<(), Error> {
        let result =
            serde_json::from_str::<Transaction>(include_str!("__TEST_DATA__/transaction.json"));

        let transaction = result.expect("Transaction is valid");
        assert_eq!(transaction.kind, OperationKind::Transaction);

        let metadata = transaction.metadata.expect("Transaction has metadata");
        assert!(metadata.operation_result.big_map_diff.is_some());
        assert!(metadata.operation_result.lazy_storage_diff.is_some());
        Ok(())
    }
}
