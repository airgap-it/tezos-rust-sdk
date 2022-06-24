use serde::{Deserialize, Serialize};

use super::error::RPCError;

#[derive(Serialize, Deserialize)]
pub struct InvalidBlock {
    pub block: String,
    pub level: u64,
    pub errors: Vec<RPCError>
}
