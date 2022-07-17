#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tezos_core::types::number::Int;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(remote = "Int")
)]
pub struct IntDef(#[cfg_attr(feature = "serde", serde(getter = "Int::to_string"))] String);

impl From<IntDef> for Int {
    fn from(value: IntDef) -> Self {
        Int::from_string(value.0).unwrap()
    }
}
