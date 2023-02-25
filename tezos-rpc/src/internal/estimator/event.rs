use crate::models::{
    error::RpcError,
    operation::operation_result::{operations::event::EventOperationResult, OperationResultStatus},
};

use super::RpcOperationResult;

impl RpcOperationResult for EventOperationResult {
    fn status(&self) -> OperationResultStatus {
        self.status
    }

    fn number_of_originated_contracts(&self) -> usize {
        0
    }

    fn consumed_gas(&self) -> num_bigint::BigUint {
        0u8.into()
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
        None
    }
}
