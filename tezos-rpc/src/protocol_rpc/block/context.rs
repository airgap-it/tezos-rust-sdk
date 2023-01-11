pub mod big_maps;
pub mod constants;
pub mod contract;
pub mod delegates;

use super::BlockId;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/context", super::path(chain_id, block_id))
}
