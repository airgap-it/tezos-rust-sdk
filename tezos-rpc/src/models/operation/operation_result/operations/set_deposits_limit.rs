use {
    crate::{
        models::error::RpcError, models::operation::kind::OperationKind,
        models::operation::operation_result::OperationResultStatus,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SetDepositsLimitOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SetDepositsLimitSuccessfulManagerOperationResult {
    /// [OperationKind::SetDepositsLimit]
    pub kind: OperationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
}
