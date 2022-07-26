pub mod run_operation;

use super::BlockId;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/scripts", super::path(chain_id, block_id))
}
