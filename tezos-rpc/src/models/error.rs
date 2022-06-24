use derive_more::{Error};
use std::fmt::Display;
use serde::{Deserialize, Serialize};

/// The error schema for all the RPC errors can be found at:
///
/// [RPC Errors](https://tezos.gitlab.io/api/errors.html)
/// or by querying [`GET /errors`](https://tezos.gitlab.io/shell/rpc.html#get-errors)
#[derive(Debug, Serialize, Deserialize, Clone, Error, PartialEq)]
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
