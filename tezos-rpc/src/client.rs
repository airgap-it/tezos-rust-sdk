use {
    crate::constants,
    crate::error::Error,
    crate::http,
    crate::models::block::Block,
    crate::models::bootstrapped_status::BootstrappedStatus,
    crate::models::checkpoint::Checkpoint,
    crate::models::invalid_block::InvalidBlock,
    crate::protocol_rpc,
    crate::protocol_rpc::block::{BlockID, MetadataRPCArg},
    crate::protocol_rpc::ProtocolRPC,
    crate::shell_rpc,
    crate::shell_rpc::chains::chain::blocks::GetBlocksQuery,
    crate::shell_rpc::injection::block::InjectionBlockPayload,
    async_trait::async_trait,
    std::result::Result,
    tezos_core::types::encoded::{BlockHash, ChainId, OperationHash},
};

pub struct TezosRPCContext {
    /// A chain identifier. This is either a chain hash in Base58Check notation or a one the predefined aliases: 'main', 'test'.
    pub chain_id: String,
    pub http_client: http::TezosHttp,
}
impl TezosRPCContext {
    /// Changes the rpc endpoint used in RPC requests.
    pub fn change_rpc_endpoint(&mut self, rpc_endpoint: &str) {
        self.http_client
            .change_rpc_endpoint(rpc_endpoint.to_string());
    }
}

pub struct TezosRPC {
    pub context: TezosRPCContext,
}

impl TezosRPC {
    /// Creates a Tezos RPC client that will connect to the specified node RPC.
    ///
    /// ```rust
    /// use tezos_rpc::client::{TezosRPC};
    ///
    /// let client = TezosRPC::new("https://tezos-node.prod.gke.papers.tech");
    /// ```
    pub fn new(rpc_endpoint: &str) -> Self {
        TezosRPC {
            context: TezosRPCContext {
                chain_id: constants::DEFAULT_CHAIN_ALIAS.to_string(),
                http_client: http::TezosHttp::new(rpc_endpoint),
            },
        }
    }

    /// Creates a Tezos RPC client that will connect to the specified node RPC.
    ///
    /// This method allows the user to provide the chain identifier that will be used when
    /// sending requests to the RPC. The default is `main`.
    ///
    /// ```rust
    /// use tezos_rpc::client::{TezosRPC};
    ///
    /// let client = TezosRPC::new_with_chain_id("https://tezos-node.prod.gke.papers.tech", "NetXLH1uAxK7CCh");
    /// ```
    pub fn new_with_chain_id(rpc_endpoint: &str, chain_id: &str) -> Self {
        TezosRPC {
            context: TezosRPCContext {
                chain_id: chain_id.to_string(),
                http_client: http::TezosHttp::new(rpc_endpoint),
            },
        }
    }
}

#[async_trait]
impl<'a> shell_rpc::ShellRPC for TezosRPC {
    async fn get_chain_id(&self) -> Result<ChainId, Error> {
        shell_rpc::chains::chain::chain_id::get(&self.context).await
    }

    async fn get_blocks(&self, query: &GetBlocksQuery) -> Result<Vec<Vec<BlockHash>>, Error> {
        shell_rpc::chains::chain::blocks::get(&self.context, query).await
    }

    async fn get_invalid_blocks(&self) -> Result<Vec<InvalidBlock>, Error> {
        shell_rpc::chains::chain::invalid_blocks::get(&self.context).await
    }

    async fn get_invalid_block(&self, block_hash: &BlockHash) -> Result<InvalidBlock, Error> {
        shell_rpc::chains::chain::invalid_blocks::block::get(&self.context, block_hash).await
    }

    async fn remove_invalid_block(&self, block_hash: &BlockHash) -> Result<(), Error> {
        shell_rpc::chains::chain::invalid_blocks::block::delete(&self.context, block_hash).await
    }

    async fn is_bootstrapped(&self) -> Result<BootstrappedStatus, Error> {
        shell_rpc::chains::chain::is_bootstrapped::get(&self.context).await
    }

    async fn get_caboose(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::caboose::get(&self.context).await
    }

    async fn get_checkpoint(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::checkpoint::get(&self.context).await
    }

    async fn get_savepoint(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::savepoint::get(&self.context).await
    }

    async fn inject_operation(
        &self,
        signed_operation_contents: &String,
        do_async: &bool,
    ) -> Result<OperationHash, Error> {
        shell_rpc::injection::operation::post(&self.context, signed_operation_contents, do_async)
            .await
    }

    async fn inject_block(
        &self,
        payload: &InjectionBlockPayload,
        force: &bool,
        do_async: &bool,
    ) -> Result<BlockHash, Error> {
        shell_rpc::injection::block::post(&self.context, payload, force, do_async).await
    }
}

#[async_trait]
impl<'a> ProtocolRPC for TezosRPC {
    async fn get_block(
        &self,
        block_id: Option<&BlockID>,
        metadata: MetadataRPCArg,
    ) -> Result<Block, Error> {
        protocol_rpc::block::get(&self.context, block_id, metadata).await
    }
}
