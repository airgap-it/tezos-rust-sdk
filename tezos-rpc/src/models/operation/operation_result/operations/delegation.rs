use {
    crate::{
        models::error::RpcError,
        models::operation::kind::OperationKind,
        models::operation::operation_result::{
            LazyStorageDiff, OperationResult, OperationResultStatus,
        },
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::{Address, ContractAddress, ImplicitAddress},
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

impl OperationResult for DelegationOperationResult {
    fn status(&self) -> OperationResultStatus {
        self.status
    }

    fn originated_contracts(&self) -> Option<&Vec<ContractAddress>> {
        None
    }

    fn consumed_gas(&self) -> num_bigint::BigUint {
        self.consumed_gas
            .as_ref()
            .map_or(0u8.into(), |consumed_gas| {
                consumed_gas.parse().unwrap_or(0u8.into())
            })
    }

    fn consumed_milligas(&self) -> num_bigint::BigUint {
        self.consumed_milligas
            .as_ref()
            .map_or(0u8.into(), |consumed_gas| {
                consumed_gas.parse().unwrap_or(0u8.into())
            })
    }

    fn paid_storage_size_diff(&self) -> Option<num_bigint::BigUint> {
        None
    }

    fn allocated_destination_contract(&self) -> Option<bool> {
        None
    }

    fn lazy_storage_diff(&self) -> Option<&Vec<LazyStorageDiff>> {
        None
    }
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
