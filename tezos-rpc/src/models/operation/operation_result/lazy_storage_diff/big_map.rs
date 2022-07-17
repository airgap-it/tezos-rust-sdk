use tezos_core::types::encoded::ScriptExprHash;

use {
    super::Kind,
    crate::models::operation::operation_result::DiffAction,
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BigMap {
    pub kind: Kind,
    pub id: String,
    pub diff: Diff,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Diff {
    pub action: DiffAction,
    pub updates: Vec<Update>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_hash: Option<ScriptExprHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<Micheline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<Micheline>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Update {
    pub key_hash: ScriptExprHash,
    pub key: Micheline,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Micheline>,
}
