use {
    crate::{models::error::RpcError, models::operation::operation_result::OperationResultStatus},
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::TxRollupId,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TxRollupOriginationOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_rollup: Option<TxRollupId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}
