use async_trait::async_trait;
use std::result::Result;
use tezos_core::types::encoded::{ChainID};
use crate::models::checkpoint::{Checkpoint};
use crate::error::{Error};

/// Tezos protocol-dependent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/active/rpc.html) for more details.
#[async_trait]
pub trait ActiveRPC {
}

/// Tezos protocol-independent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/shell/rpc.html) for more details.
#[async_trait]
pub trait ShellRPC {
    /// Get the chain unique identifier.
    ///
    /// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
    async fn chain_id(&self) -> Result<ChainID, Error>;

    /// Get the current checkpoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
    async fn checkpoint(&self) -> Result<Checkpoint, Error>;
}
