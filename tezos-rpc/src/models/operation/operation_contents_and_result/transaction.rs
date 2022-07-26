use crate::models::operation::operation_result::operations::InternalOperationResult;

use crate::{Error, Result};

use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::transaction::TransactionOperationResult,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{Address, ImplicitAddress},
        mutez::Mutez,
    },
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

impl From<tezos_operation::operations::Transaction> for Transaction {
    fn from(value: tezos_operation::operations::Transaction) -> Self {
        Self {
            kind: OperationKind::Transaction,
            source: value.source,
            fee: value.fee,
            counter: value.counter.into(),
            gas_limit: value.gas_limit.into(),
            storage_limit: value.storage_limit.into(),
            amount: value.amount,
            destination: value.destination,
            parameters: value.parameters.map(|parameters| parameters.into()),
            metadata: None,
        }
    }
}

impl TryFrom<Transaction> for tezos_operation::operations::Transaction {
    type Error = Error;

    fn try_from(value: Transaction) -> Result<Self> {
        Ok(Self {
            source: value.source,
            fee: value.fee,
            counter: value.counter.try_into()?,
            gas_limit: value.gas_limit.try_into()?,
            storage_limit: value.storage_limit.try_into()?,
            amount: value.amount,
            destination: value.destination,
            parameters: value.parameters.map(|parameters| parameters.into()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionMetadata {
    pub operation_result: TransactionOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
    #[serde(default)]
    pub internal_operation_results: Vec<InternalOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TransactionParameters {
    pub entrypoint: Entrypoint,
    pub value: Micheline,
}

impl From<tezos_operation::operations::Parameters> for TransactionParameters {
    fn from(value: tezos_operation::operations::Parameters) -> Self {
        Self {
            entrypoint: value.entrypoint.into(),
            value: value.value,
        }
    }
}

impl From<TransactionParameters> for tezos_operation::operations::Parameters {
    fn from(value: TransactionParameters) -> Self {
        Self {
            entrypoint: value.entrypoint.into(),
            value: value.value,
        }
    }
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
            _ => Entrypoint::Named(value.into()),
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

impl From<tezos_operation::operations::Entrypoint> for Entrypoint {
    fn from(value: tezos_operation::operations::Entrypoint) -> Self {
        match value {
            tezos_operation::operations::Entrypoint::Default => Self::Default,
            tezos_operation::operations::Entrypoint::Root => Self::Root,
            tezos_operation::operations::Entrypoint::Do => Self::Do,
            tezos_operation::operations::Entrypoint::SetDelegate => Self::SetDelegate,
            tezos_operation::operations::Entrypoint::RemoveDelegate => Self::RemoveDelegate,
            tezos_operation::operations::Entrypoint::Named(value) => Self::Named(value),
        }
    }
}

impl From<Entrypoint> for tezos_operation::operations::Entrypoint {
    fn from(value: Entrypoint) -> Self {
        match value {
            Entrypoint::Default => Self::Default,
            Entrypoint::Root => Self::Root,
            Entrypoint::Do => Self::Do,
            Entrypoint::SetDelegate => Self::SetDelegate,
            Entrypoint::RemoveDelegate => Self::RemoveDelegate,
            Entrypoint::Named(value) => Self::Named(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transaction_deserialization() -> Result<()> {
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
