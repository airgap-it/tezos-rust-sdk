use {
    crate::{
        models::error::RPCError,
        models::operation::operation_result::OperationResultStatus,
        models::operation::kind::OperationKind,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DelegationOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DelegationSuccessfulManagerOperationResult {
    /// [OperationKind::Delegation]
    pub kind: OperationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
}
