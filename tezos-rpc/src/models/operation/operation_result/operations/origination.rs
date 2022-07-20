use {
    crate::models::{
        balance_update::BalanceUpdate,
        contract::ContractScript,
        error::RpcError,
        operation::kind::OperationKind,
        operation::operation_result::{
            big_map_diff::BigMapDiff, lazy_storage_diff::LazyStorageDiff,
        },
        operation::operation_result::{OperationResult, OperationResultStatus},
    },
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::{Address, ContractAddress, ImplicitAddress},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginationOperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<BigMapDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_contracts: Option<Vec<ContractAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_storage_size_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<Vec<LazyStorageDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}

impl OperationResult for OriginationOperationResult {
    fn status(&self) -> OperationResultStatus {
        self.status
    }

    fn originated_contracts(&self) -> Option<&Vec<ContractAddress>> {
        self.originated_contracts.as_ref()
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
        self.paid_storage_size_diff
            .as_ref()
            .map(|consumed_gas| consumed_gas.parse().unwrap_or(0u8.into()))
    }

    fn allocated_destination_contract(&self) -> Option<bool> {
        None
    }

    fn lazy_storage_diff(&self) -> Option<&Vec<LazyStorageDiff>> {
        self.lazy_storage_diff.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginationSuccessfulManagerOperationResult {
    /// [OperationKind::Origination]
    pub kind: OperationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<BigMapDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_contracts: Option<Vec<ContractAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_storage_size_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<LazyStorageDiff>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InternalOriginationOperationResult {
    /// [OperationKind::Origination]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: Address,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Mutez
    pub balance: u64,
    /// Address (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ContractScript>,
    pub result: OriginationOperationResult,
}
