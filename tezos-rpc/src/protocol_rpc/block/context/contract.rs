pub mod balance;
pub mod counter;

fn path(chain_id: String, block_id: String, contract: String) -> String {
    format!("{}/contracts/{}", super::path(chain_id, block_id), contract)
}
