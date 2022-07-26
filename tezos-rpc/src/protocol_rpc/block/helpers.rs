pub mod preapply;
pub mod scripts;

use super::BlockId;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId) -> String {
    format!("{}/helpers", super::path(chain_id, block_id))
}
