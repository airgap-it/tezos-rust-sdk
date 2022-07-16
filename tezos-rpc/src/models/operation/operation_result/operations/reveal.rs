use crate::models::operation::kind::OperationKind;

use {
    crate::{models::error::RpcError, models::operation::operation_result::OperationResultStatus},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RevealOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RevealSuccessfulManagerOperationResult {
    /// [OperationKind::Reveal]
    pub kind: OperationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
}
