use derive_more::Error;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The error schema for all the RPC errors can be found at:
///
/// [RPC Errors](https://tezos.gitlab.io/api/errors.html)
/// or by querying [`GET /errors`](https://tezos.gitlab.io/shell/rpc.html#get-errors)
#[derive(Debug, Serialize, Deserialize, Clone, Error, PartialEq)]
pub struct RpcError {
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

impl Display for RpcError {
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

#[derive(Debug, Clone)]
pub struct RpcErrors(Vec<RpcError>);

impl From<Vec<RpcError>> for RpcErrors {
    fn from(value: Vec<RpcError>) -> Self {
        Self(value)
    }
}

impl Display for RpcErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for item in &self.0 {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}
