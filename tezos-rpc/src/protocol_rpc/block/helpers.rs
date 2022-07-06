pub mod preapply;
pub mod scripts;

use super::BlockID;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/helpers", super::path(chain_id, block_id))
}
