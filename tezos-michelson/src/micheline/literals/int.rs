use serde::{Deserialize, Serialize};
use tezos_core::types::number::integer::Integer;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Integer")]
pub struct IntDef(#[serde(getter = "Integer::to_string")] String);

impl From<IntDef> for Integer {
    fn from(value: IntDef) -> Self {
        Integer::from_string(value.0).unwrap()
    }
}
