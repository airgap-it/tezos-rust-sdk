pub mod backtracked;
pub mod big_map_diff;
pub mod lazy_storage_diff;
pub mod operations;

use serde::{Deserialize, Serialize};

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
