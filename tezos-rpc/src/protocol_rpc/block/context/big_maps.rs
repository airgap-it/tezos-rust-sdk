pub mod big_map;

use crate::models::block::BlockId;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/big_maps", super::path(chain_id, block_id))
}
