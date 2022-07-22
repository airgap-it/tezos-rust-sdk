use {
    crate::models::{
        balance_update::BalanceUpdate, operation::kind::OperationKind,
        operation::operation_result::operations::set_deposits_limit::SetDepositsLimitOperationResult,
    },
    crate::{Error, Result},
    serde::{Deserialize, Serialize},
    tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositsLimit {
    /// [OperationKind::SetDepositsLimit]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Mutez>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SetDepositsLimitMetadata>,
}

impl From<tezos_operation::operations::SetDepositsLimit> for SetDepositsLimit {
    fn from(value: tezos_operation::operations::SetDepositsLimit) -> Self {
        Self {
            kind: OperationKind::SetDepositsLimit,
            source: value.source,
            fee: value.fee,
            counter: value.counter.into(),
            gas_limit: value.gas_limit.into(),
            storage_limit: value.storage_limit.into(),
            limit: value.limit,
            metadata: None,
        }
    }
}

impl TryFrom<SetDepositsLimit> for tezos_operation::operations::SetDepositsLimit {
    type Error = Error;

    fn try_from(value: SetDepositsLimit) -> Result<Self> {
        Ok(Self {
            source: value.source,
            fee: value.fee,
            counter: value.counter.try_into()?,
            gas_limit: value.gas_limit.try_into()?,
            storage_limit: value.storage_limit.try_into()?,
            limit: value.limit,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositsLimitMetadata {
    pub operation_result: SetDepositsLimitOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
