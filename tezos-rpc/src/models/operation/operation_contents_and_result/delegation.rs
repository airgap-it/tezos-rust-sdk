use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez};

use crate::{Error, Result};

use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::delegation::DelegationOperationResult,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Delegation {
    /// [OperationKind::Delegation]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DelegationMetadata>,
}

impl From<tezos_operation::operations::Delegation> for Delegation {
    fn from(value: tezos_operation::operations::Delegation) -> Self {
        Self {
            kind: OperationKind::Delegation,
            source: value.source,
            fee: value.fee,
            counter: value.counter.into(),
            gas_limit: value.gas_limit.into(),
            storage_limit: value.storage_limit.into(),
            delegate: value.delegate,
            metadata: None,
        }
    }
}

impl TryFrom<Delegation> for tezos_operation::operations::Delegation {
    type Error = Error;

    fn try_from(value: Delegation) -> Result<Self> {
        Ok(Self {
            source: value.source,
            fee: value.fee,
            counter: value.counter.try_into()?,
            gas_limit: value.gas_limit.try_into()?,
            storage_limit: value.storage_limit.try_into()?,
            delegate: value.delegate,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelegationMetadata {
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
    pub operation_result: DelegationOperationResult,
}
