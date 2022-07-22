use {
    crate::{
        models::balance_update::BalanceUpdate, models::contract::ContractScript,
        models::operation::kind::OperationKind,
        models::operation::operation_result::operations::origination::OriginationOperationResult,
    },
    crate::{Error, Result},
    serde::{Deserialize, Serialize},
    tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Origination {
    /// [OperationKind::Origination]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub balance: Mutez,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    pub script: ContractScript,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OriginationMetadata>,
}

impl From<tezos_operation::operations::Origination> for Origination {
    fn from(value: tezos_operation::operations::Origination) -> Self {
        Self {
            kind: OperationKind::Origination,
            source: value.source,
            fee: value.fee,
            counter: value.counter.into(),
            gas_limit: value.gas_limit.into(),
            storage_limit: value.storage_limit.into(),
            balance: value.balance,
            delegate: value.delegate,
            script: value.script.into(),
            metadata: None,
        }
    }
}

impl TryFrom<Origination> for tezos_operation::operations::Origination {
    type Error = Error;

    fn try_from(value: Origination) -> Result<Self> {
        Ok(Self {
            source: value.source,
            fee: value.fee,
            counter: value.counter.try_into()?,
            gas_limit: value.gas_limit.try_into()?,
            storage_limit: value.storage_limit.try_into()?,
            balance: value.balance,
            delegate: value.delegate,
            script: value.script.into(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OriginationMetadata {
    pub operation_result: OriginationOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
