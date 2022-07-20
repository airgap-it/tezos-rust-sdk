use {
    crate::{
        models::error::RpcError, models::operation::kind::OperationKind,
        models::operation::operation_result::OperationResultStatus,
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::{Address, ImplicitAddress},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DelegationOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InternalDelegationOperationResult {
    /// [OperationKind::Delegation]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: Address,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    pub result: DelegationOperationResult,
}
