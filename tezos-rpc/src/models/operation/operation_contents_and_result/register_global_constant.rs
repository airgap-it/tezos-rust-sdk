use {
    crate::{
        models::balance_update::BalanceUpdate, models::error::RPCError,
        models::operation::kind::Kind, models::operation::operation_result::Status,
    },
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterGlobalConstant {
    /// [Kind::RegisterGlobalConstant]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    pub value: Micheline, // FIXME: Should be Michelson
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RegisterGlobalConstantMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterGlobalConstantMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: RegisterGlobalConstantOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<InternalRegisterGlobalConstantOperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterGlobalConstantOperationResult {
    pub status: Status,
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    /// Script expression (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalRegisterGlobalConstantOperationResult {
    /// [Kind::RegisterGlobalConstant]
    pub kind: Kind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    pub value: Micheline,
    pub result: RegisterGlobalConstantOperationResult,
}
