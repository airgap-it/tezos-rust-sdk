use num_bigint::BigUint;
use tezos_core::types::encoded::ContractAddress;

pub mod backtracked;
pub mod big_map_diff;
pub mod lazy_storage_diff;
pub mod operations;

use {
    self::{
        big_map_diff::BigMapDiff, lazy_storage_diff::big_map::BigMap,
        lazy_storage_diff::LazyStorageDiff,
    },
    crate::models::limits::{OperationLimits, GAS_SAFETY_MARGIN, STORAGE_SAFETY_MARGIN},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationResultStatus {
    Applied,
    Failed,
    Skipped,
    Backtracked,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiffAction {
    Update,
    Remove,
    Copy,
    Alloc,
}

pub trait OperationResult {
    fn status(&self) -> OperationResultStatus;

    fn originated_contracts(&self) -> Option<&Vec<ContractAddress>>;

    fn consumed_gas(&self) -> BigUint;

    fn consumed_milligas(&self) -> BigUint;

    fn paid_storage_size_diff(&self) -> Option<BigUint>;

    fn allocated_destination_contract(&self) -> Option<bool>;

    fn lazy_storage_diff(&self) -> Option<&Vec<LazyStorageDiff>>;

    fn big_map_diffs(&self, matching_indices: Option<&[u32]>) -> Vec<&BigMap> {
        match self.status() {
            OperationResultStatus::Applied => self
                .lazy_storage_diff()
                .map(|diff| {
                    diff.into_iter()
                        .flat_map(|diff| match diff {
                            LazyStorageDiff::BigMap(big_map) => {
                                if let Some(matching_indices) = matching_indices {
                                    big_map
                                        .id
                                        .parse::<u32>()
                                        .map(|index| {
                                            if matching_indices.contains(&index) {
                                                Some(big_map)
                                            } else {
                                                None
                                            }
                                        })
                                        .unwrap_or(None)
                                } else {
                                    Some(big_map)
                                }
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or(vec![]),
            _ => vec![],
        }
    }

    fn limits(&self) -> OperationLimits {
        OperationLimits {
            gas: self.consumed_gas() + GAS_SAFETY_MARGIN,
            storage: self.paid_storage_size_diff().unwrap_or(0u8.into())
                + self.burn_fee()
                + STORAGE_SAFETY_MARGIN,
        }
    }

    // fn consumed_gas(&self) -> BigUint {
    //     self.consumed_gas
    //         .as_ref()
    //         .map(|consumed_gas| consumed_gas.parse::<BigUint>().unwrap_or(0u8.into()))
    //         .unwrap_or(0u8.into())
    // }

    // fn paid_storage_size_diff(&self) -> BigUint {
    //     self.paid_storage_size_diff
    //         .as_ref()
    //         .map(|paid_storage_diff| paid_storage_diff.parse::<BigUint>().unwrap_or(0u8.into()))
    //         .unwrap_or(0u8.into())
    // }

    fn burn_fee(&self) -> BigUint {
        let mut sum: BigUint = 0u8.into();
        if let Some(allocated) = self.allocated_destination_contract() {
            if allocated {
                sum += 257u16;
            }
        }

        if let Some(contracts) = &self.originated_contracts() {
            sum += (contracts.len() as u64) * 257
        }

        return sum;
    }
}
