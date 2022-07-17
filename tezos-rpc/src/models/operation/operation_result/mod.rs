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
    crate::{
        models::balance_update::BalanceUpdate,
        models::error::RpcError,
        models::limits::{Limits, GAS_SAFETY_MARGIN, STORAGE_SAFETY_MARGIN},
    },
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OperationResult {
    pub status: OperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<Vec<BigMapDiff>>,
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
    pub allocated_destination_contract: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<Vec<LazyStorageDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RpcError>>,
}

impl OperationResult {
    pub fn big_map_diffs(&self, matching_indices: Option<&[u32]>) -> Vec<&BigMap> {
        match self.status {
            OperationResultStatus::Applied => self
                .lazy_storage_diff
                .as_ref()
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

    pub fn limits(&self) -> Limits {
        Limits {
            gas_limit: self.consumed_gas() + GAS_SAFETY_MARGIN,
            storage_limit: self.paid_storage_size_diff() + self.burn_fee() + STORAGE_SAFETY_MARGIN,
        }
    }

    pub fn consumed_gas(&self) -> u64 {
        self.consumed_gas
            .as_ref()
            .map(|consumed_gas| consumed_gas.parse::<u64>().unwrap_or(0))
            .unwrap_or(0)
    }

    pub fn paid_storage_size_diff(&self) -> u64 {
        self.paid_storage_size_diff
            .as_ref()
            .map(|paid_storage_diff| paid_storage_diff.parse::<u64>().unwrap_or(0))
            .unwrap_or(0)
    }

    pub fn burn_fee(&self) -> u64 {
        let mut sum = 0u64;
        if let Some(allocated) = self.allocated_destination_contract {
            if allocated {
                sum += 257;
            }
        }

        if let Some(contracts) = &self.originated_contracts {
            sum += (contracts.len() as u64) * 257
        }

        return sum;
    }
}
