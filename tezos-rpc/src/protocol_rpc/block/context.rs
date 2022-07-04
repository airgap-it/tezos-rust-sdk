pub mod contract;

fn path(chain_id: String, block_id: String) -> String {
    format!("{}/context", super::path(chain_id, block_id))
}
