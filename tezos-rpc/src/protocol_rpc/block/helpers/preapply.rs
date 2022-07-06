pub mod operations;

use super::BlockID;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID) -> String {
    format!("{}/preapply", super::path(chain_id, block_id))
}
