pub mod run_operation;

use super::BlockID;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/scripts", super::path(chain_id, block_id))
}
