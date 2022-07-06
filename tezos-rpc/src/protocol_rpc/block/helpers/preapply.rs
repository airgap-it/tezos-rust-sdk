pub mod operations;

use super::BlockID;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/preapply", super::path(chain_id, block_id))
}
