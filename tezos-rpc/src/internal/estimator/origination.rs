use crate::models::{
    error::RpcError,
    operation::{
        operation_contents_and_result::origination::OriginationMetadata,
        operation_result::{
            operations::{origination::OriginationOperationResult, InternalOperationResult},
            OperationResultStatus,
        },
    },
};

use super::{RpcMetadata, RpcOperationResult};

impl RpcOperationResult for OriginationOperationResult {
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
        None
    }

    fn errors(&self) -> Option<&Vec<RpcError>> {
        self.errors.as_ref()
    }
}

impl RpcMetadata<OriginationOperationResult> for OriginationMetadata {
    fn operation_result(&self) -> &OriginationOperationResult {
        &self.operation_result
    }

    fn internal_operation_results(&self) -> Option<&Vec<InternalOperationResult>> {
        None
    }
}
