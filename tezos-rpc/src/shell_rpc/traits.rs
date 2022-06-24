use async_trait::async_trait;
use std::result::Result;
use tezos_core::types::encoded::ChainID;
use crate::models::checkpoint::Checkpoint;
use crate::error::{Error};
use crate::models::invalid_block::InvalidBlock;

/// Tezos protocol-independent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/shell/rpc.html) for more details.
#[async_trait]
pub trait ShellRPC {
    /// Get the chain unique identifier.
    ///
    /// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
    async fn chain_id(&self) -> Result<ChainID, Error>;

    /// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
    ///
    /// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
    async fn invalid_blocks(&self) -> Result<Vec<InvalidBlock>, Error>;

    /// Get the current checkpoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
    async fn checkpoint(&self) -> Result<Checkpoint, Error>;
}
