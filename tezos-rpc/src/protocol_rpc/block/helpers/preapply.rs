pub mod operations;

use super::BlockId;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/preapply", super::path(chain_id, block_id))
}
