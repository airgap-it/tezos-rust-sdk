use crate::models::{
    error::RpcError,
    operation::{
        operation_contents_and_result::transaction::TransactionMetadata,
        operation_result::{
            operations::{transaction::TransactionOperationResult, InternalOperationResult},
            OperationResultStatus,
        },
    },
};

use super::{RpcMetadata, RpcOperationResult};

impl RpcOperationResult for TransactionOperationResult {
    fn status(&self) -> OperationResultStatus {
        self.status
    }

    fn number_of_originated_contracts(&self) -> usize {
        self.originated_contracts
            .as_ref()
            .map_or(0, |contracts| contracts.len())
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
        self.allocated_destination_contract
    }

    fn errors(&self) -> Option<&Vec<RpcError>> {
        self.errors.as_ref()
    }
}

impl RpcMetadata<TransactionOperationResult> for TransactionMetadata {
    fn operation_result(&self) -> &TransactionOperationResult {
        &self.operation_result
    }

    fn internal_operation_results(&self) -> Option<&Vec<InternalOperationResult>> {
        Some(&self.internal_operation_results)
    }
}
