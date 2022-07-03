use serde::{Deserialize, Serialize};

use self::{big_map::BigMap, sapling_state::SaplingState};

pub mod big_map;
pub mod sapling_state;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum LazyStorageDiff {
    BigMap(BigMap),
    SaplingState(SaplingState),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    BigMap,
    SaplingState,
}
