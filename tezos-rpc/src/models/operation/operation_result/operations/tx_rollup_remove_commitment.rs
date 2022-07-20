use {
    crate::{models::error::RpcError, models::operation::operation_result::OperationResultStatus},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TxRollupRemoveCommitmentOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}
