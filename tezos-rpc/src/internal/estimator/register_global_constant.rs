use crate::models::{
    error::RpcError,
    operation::{
        operation_contents_and_result::register_global_constant::RegisterGlobalConstantMetadata,
        operation_result::{
            operations::{
                register_global_constant::RegisterGlobalConstantOperationResult,
                InternalOperationResult,
            },
            OperationResultStatus,
        },
    },
};

use super::{RpcMetadata, RpcOperationResult};

impl RpcOperationResult for RegisterGlobalConstantOperationResult {
    fn status(&self) -> OperationResultStatus {
        self.status
    }

    fn number_of_originated_contracts(&self) -> usize {
        0
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

    fn errors(&self) -> Option<&Vec<RpcError>> {
        self.errors.as_ref()
    }
}

impl RpcMetadata<RegisterGlobalConstantOperationResult> for RegisterGlobalConstantMetadata {
    fn operation_result(&self) -> &RegisterGlobalConstantOperationResult {
        &self.operation_result
    }

    fn internal_operation_results(&self) -> Option<&Vec<InternalOperationResult>> {
        None
    }
}
