use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Checkpoint {
    pub block_hash: String,
    pub level: u64
}
