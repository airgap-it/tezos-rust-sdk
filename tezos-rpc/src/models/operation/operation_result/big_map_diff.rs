use tezos_core::types::encoded::ScriptExprHash;

use {
    super::DiffAction,
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BigMapDiff {
    pub action: DiffAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_big_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_big_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_hash: Option<ScriptExprHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<Micheline>,
}
