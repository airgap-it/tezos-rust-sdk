use {
    crate::{
        models::error::RPCError,
        models::balance_update::BalanceUpdate,
        models::operation::operation_result::OperationResultStatus,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegisterGlobalConstantOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    /// Script expression (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}
