pub mod big_maps;
pub mod constants;
pub mod contract;

use super::BlockID;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID) -> String {
    format!("{}/context", super::path(chain_id, block_id))
}
