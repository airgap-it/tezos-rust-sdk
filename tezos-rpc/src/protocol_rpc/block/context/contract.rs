use crate::protocol_rpc::block::BlockID;

pub mod balance;
pub mod counter;

fn path(chain_id: &String, block_id: &BlockID, contract: &String) -> String {
    format!("{}/contracts/{}", super::path(chain_id, block_id), contract)
}
