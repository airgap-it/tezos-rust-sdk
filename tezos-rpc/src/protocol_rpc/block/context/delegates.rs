pub mod voting_info;

use crate::protocol_rpc::block::BlockId;

fn path<S: AsRef<str>>(chain_id: S, block_id: &BlockId, contract: S) -> String {
    format!(
        "{}/delegates/{}",
        super::path(chain_id, block_id),
        contract.as_ref()
    )
}
