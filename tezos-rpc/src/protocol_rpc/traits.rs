use {
    super::block::{BlockID, MetadataRPCArg},
    crate::error::Error,
    crate::models::block::Block,
    async_trait::async_trait,
    std::result::Result,
};

/// Tezos protocol-dependent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/active/rpc.html) for more details.
#[async_trait]
pub trait ProtocolRPC {
    /// Get all the information about a block.
    /// The associated metadata may not be present depending on the history mode and block's distance from the head.
    ///
    /// [`GET /chains/<chain_id>/blocks/<block_id>?[force_metadata]&[metadata=<metadata_rpc_arg>]`](https://tezos.gitlab.io/active/rpc.html#get-block-id)
    async fn get_block(
        &self,
        block_id: &Option<BlockID>,
        metadata: MetadataRPCArg,
    ) -> Result<Block, Error>;
}
