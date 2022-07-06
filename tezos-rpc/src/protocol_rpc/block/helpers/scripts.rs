pub mod run_operation;

use super::BlockID;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID) -> String {
    format!("{}/scripts", super::path(chain_id, block_id))
}
