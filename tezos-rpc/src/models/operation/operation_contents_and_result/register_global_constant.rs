use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::register_global_constant::RegisterGlobalConstantOperationResult,
    },
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterGlobalConstantMetadata {
    pub operation_result: RegisterGlobalConstantOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}
