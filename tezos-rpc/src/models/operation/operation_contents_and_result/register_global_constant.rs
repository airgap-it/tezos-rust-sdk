use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::register_global_constant::RegisterGlobalConstantOperationResult,
    },
    crate::{Error, Result},
    serde::{Deserialize, Serialize},
    tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez},
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterGlobalConstant {
    /// [OperationKind::RegisterGlobalConstant]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub value: Micheline, // FIXME: Should be Michelson
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RegisterGlobalConstantMetadata>,
}

impl From<tezos_operation::operations::RegisterGlobalConstant> for RegisterGlobalConstant {
    fn from(value: tezos_operation::operations::RegisterGlobalConstant) -> Self {
        Self {
            kind: OperationKind::RegisterGlobalConstant,
            source: value.source,
            fee: value.fee,
            counter: value.counter.into(),
            gas_limit: value.gas_limit.into(),
            storage_limit: value.storage_limit.into(),
            value: value.value,
            metadata: None,
        }
    }
}

impl TryFrom<RegisterGlobalConstant> for tezos_operation::operations::RegisterGlobalConstant {
    type Error = Error;

    fn try_from(value: RegisterGlobalConstant) -> Result<Self> {
        Ok(Self {
            source: value.source,
            fee: value.fee,
            counter: value.counter.try_into()?,
            gas_limit: value.gas_limit.try_into()?,
            storage_limit: value.storage_limit.try_into()?,
            value: value.value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterGlobalConstantMetadata {
    pub operation_result: RegisterGlobalConstantOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
