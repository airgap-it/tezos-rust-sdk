use super::BlockID;

pub mod contract;
pub mod constants;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/context", super::path(chain_id, block_id))
}
