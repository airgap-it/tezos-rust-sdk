use super::BlockID;

pub mod constants;
pub mod contract;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/context", super::path(chain_id, block_id))
}
