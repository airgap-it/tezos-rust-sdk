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
    crate::shell_rpc,
    crate::shell_rpc::chains::chain::blocks::GetBlocksQuery,
    crate::shell_rpc::injection::block::InjectionBlockPayload,
    num_bigint::BigInt,
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

// Tezos protocol-independent RPCs
// See [RPCs - Reference](https://tezos.gitlab.io/shell/rpc.html) for more details.
impl TezosRPC {
    /// Get the chain unique identifier.
    ///
    /// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
    pub async fn get_chain_id(&self) -> Result<ChainId, Error> {
        shell_rpc::chains::chain::chain_id::get(&self.context).await
    }

    /// Get a list of block hashes from `<chain>`, up to the last checkpoint, sorted with
    /// decreasing fitness. Without arguments it returns the head of the chain.
    ///
    /// Optional arguments allow to return the list of predecessors of a given block or of a set of blocks.
    ///
    /// [`GET /chains/<chain_id>/blocks`](https://tezos.gitlab.io/shell/rpc.html#get_chains__chain_id__blocks)
    pub async fn get_blocks(&self, query: &GetBlocksQuery) -> Result<Vec<Vec<BlockHash>>, Error> {
        shell_rpc::chains::chain::blocks::get(&self.context, query).await
    }

    /// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
    ///
    /// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
    pub async fn get_invalid_blocks(&self) -> Result<Vec<InvalidBlock>, Error> {
        shell_rpc::chains::chain::invalid_blocks::get(&self.context).await
    }

    /// Get the errors that appeared during the block (in)validation.
    ///
    /// [`GET /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks-block-hash)
    pub async fn get_invalid_block(&self, block_hash: &BlockHash) -> Result<InvalidBlock, Error> {
        shell_rpc::chains::chain::invalid_blocks::block::get(&self.context, block_hash).await
    }

    /// Remove an invalid block for the tezos storage.
    ///
    /// [`DELETE /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#delete-chains-chain-id-invalid-blocks-block-hash)
    pub async fn remove_invalid_block(&self, block_hash: &BlockHash) -> Result<(), Error> {
        shell_rpc::chains::chain::invalid_blocks::block::delete(&self.context, block_hash).await
    }

    /// Get the bootstrap status of a chain.
    ///
    /// [`DELETE /chains/<chain_id>/is_bootstrapped`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-is-bootstrapped)
    pub async fn is_bootstrapped(&self) -> Result<BootstrappedStatus, Error> {
        shell_rpc::chains::chain::is_bootstrapped::get(&self.context).await
    }

    /// Get the current caboose for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/caboose`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-caboose)
    pub async fn get_caboose(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::caboose::get(&self.context).await
    }

    /// Get the current checkpoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
    pub async fn get_checkpoint(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::checkpoint::get(&self.context).await
    }

    /// Get the current savepoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/savepoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-savepoint)
    pub async fn get_savepoint(&self) -> Result<Checkpoint, Error> {
        shell_rpc::chains::chain::levels::savepoint::get(&self.context).await
    }

    /// Inject an operation in node and broadcast it.
    ///
    /// The `signed_operation_contents` should be constructed using contextual RPCs
    /// from the latest block and signed by the client.
    ///
    /// The injection of the operation will apply it on the current mempool context.
    /// This context may change at each operation injection or operation reception from peers.
    ///
    /// By default, the RPC will wait for the operation to be (pre-)validated before returning.
    /// However, if `?async` is true, the function returns immediately.
    /// The optional `?chain` parameter can be used to specify whether to inject on the test chain or the main chain.
    ///
    /// Returns the ID of the operation.
    ///
    /// [`POST /injection/operation?[async]&[chain=<chain_id>]`](https://tezos.gitlab.io/shell/rpc.html#post-injection-operation)
    pub async fn inject_operation(
        &self,
        signed_operation_contents: &String,
        do_async: &bool,
    ) -> Result<OperationHash, Error> {
        shell_rpc::injection::operation::post(&self.context, signed_operation_contents, do_async)
            .await
    }

    /// Inject a block in the node and broadcast it.
    ///
    /// The `operations` might be pre-validated using a contextual RPCs
    /// from the latest block (e.g. `/blocks/head/context/preapply`).
    ///
    /// By default, the RPC will wait for the block to be validated before answering.
    /// If `?async` is true, the function returns immediately. Otherwise, the block will be validated before the result is returned. If ?force is true, it will be injected even on non strictly increasing fitness. An optional ?chain parameter can be used to specify whether to inject on the test chain or the main chain.
    ///
    /// Returns the ID of the block [BlockHash].
    ///
    /// [`POST /injection/block?[async]&[force]&[chain=<chain_id>]]`](https://tezos.gitlab.io/shell/rpc.html#post-injection-block)
    pub async fn inject_block(
        &self,
        payload: &InjectionBlockPayload,
        force: &bool,
        do_async: &bool,
    ) -> Result<BlockHash, Error> {
        shell_rpc::injection::block::post(&self.context, payload, force, do_async).await
    }
}

// Tezos protocol-dependent RPCs
// See [RPCs - Reference](https://tezos.gitlab.io/active/rpc.html) for more details.
impl TezosRPC {
    /// Get all the information about a block.
    /// The associated metadata may not be present depending on the history mode and block's distance from the head.
    ///
    /// [`GET /chains/<chain_id>/blocks/<block_id>?[force_metadata]&[metadata=<metadata_rpc_arg>]`](https://tezos.gitlab.io/active/rpc.html#get-block-id)
    pub async fn get_block(
        &self,
        block_id: Option<&BlockID>,
        metadata: MetadataRPCArg,
    ) -> Result<Block, Error> {
        protocol_rpc::block::get(&self.context, block_id, metadata).await
    }

    /// Access the balance of a contract.
    ///
    /// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/balance`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-balance)
    pub async fn get_balance(&self, address: &String) -> Result<BigInt, Error> {
        protocol_rpc::block::context::contract::balance::get(&self.context, address)
            .send()
            .await
    }

    /// Access the counter of a contract.
    ///
    /// [`GET /chains/<chain_id>/blocks/<block>/context/contracts/<contract_id>/counter`](https://tezos.gitlab.io/active/rpc.html#get-block-id-context-contracts-contract-id-counter)
    pub async fn get_counter(&self, address: &String) -> Result<BigInt, Error> {
        protocol_rpc::block::context::contract::counter::get(&self.context, address)
            .send()
            .await
    }
}
