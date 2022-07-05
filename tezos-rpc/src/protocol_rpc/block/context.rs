pub mod big_maps;
pub mod constants;
pub mod contract;

use super::BlockID;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/context", super::path(chain_id, block_id))
}
