use derive_more::{Display, Error as DError, From};
use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, DError, PartialEq)]
pub struct RPCError {
    pub kind: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
            self.message.as_ref().unwrap_or(&String::from(""))
        )
    }
}

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Core { source: tezos_core::Error },
    HTTPError { source: reqwest::Error },
    ParsingError { source: serde_json::Error},
    RPCError(RPCError),
}
