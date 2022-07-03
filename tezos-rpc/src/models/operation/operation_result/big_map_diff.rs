use serde::{Deserialize, Serialize};

use tezos_michelson::micheline::Micheline;

use super::Action;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BigMapDiff {
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_big_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_big_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<Micheline>,
}
