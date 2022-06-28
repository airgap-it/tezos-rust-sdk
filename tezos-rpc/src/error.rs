use derive_more::{Error as DError, From};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, DError, PartialEq)]
pub struct RPCError {
    pub kind: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

impl Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.kind,
            self.id,
            self.message
                .as_ref()
                .unwrap_or(self.msg.as_ref().unwrap_or(&String::from("")))
        )
    }
}

#[derive(Debug, From)]
pub enum Error {
    Core { source: tezos_core::Error },
    HTTPError { source: reqwest::Error },
    ParsingError { source: serde_json::Error },
    RPCErrorPlain(String),
    RPCErrors(Vec<RPCError>),
}
