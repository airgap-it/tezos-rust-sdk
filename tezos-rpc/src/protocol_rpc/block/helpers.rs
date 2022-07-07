pub mod preapply;
pub mod scripts;

use super::BlockID;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockID) -> String {
    format!("{}/helpers", super::path(chain_id, block_id))
}
