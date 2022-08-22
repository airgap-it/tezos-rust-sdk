use tezos_core::types::encoded::{
    BlockHash, BlockPayloadHash, ChainId, ContextHash, Encoded, ImplicitAddress, NonceHash,
    OperationListListHash, ProtocolHash, Signature,
};

use crate::{Error, Result};

use {
    super::{
        balance_update::BalanceUpdate,
        operation::{operation_result::operations::SuccessfulManagerOperationResult, Operation},
    },
    crate::constants::{BLOCK_GENESIS_ALIAS, BLOCK_HEAD_ALIAS},
    crate::serde_utils::rfc3339_timestamp,
    chrono::NaiveDateTime,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub protocol: ProtocolHash,
    pub chain_id: ChainId,
    pub hash: BlockHash,
    pub header: Header,
    pub operations: Vec<Vec<Operation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Header {
    pub level: i32,
    pub proto: u8,
    pub predecessor: BlockHash,
    #[serde(with = "rfc3339_timestamp")]
    pub timestamp: NaiveDateTime,
    pub validation_pass: u8,
    pub operations_hash: OperationListListHash,
    pub fitness: Vec<String>,
    pub context: ContextHash,
    pub payload_hash: Option<BlockPayloadHash>,
    #[serde(default)]
    pub payload_round: i32,
    #[serde(default)]
    pub priority: u16,
    #[serde(default)]
    pub proof_of_work_nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_nonce_hash: Option<NonceHash>,
    #[serde(default)]
    pub liquidity_baking_escape_vote: bool,
    #[serde(default)]
    pub liquidity_baking_toggle_vote: LiquidityBakingToggleVote,
    pub signature: Option<Signature>,
}

impl From<tezos_operation::block_header::BlockHeader> for Header {
    fn from(value: tezos_operation::block_header::BlockHeader) -> Self {
        Self {
            level: value.level,
            proto: value.proto,
            predecessor: value.predecessor,
            timestamp: value.timestamp,
            validation_pass: value.validation_pass,
            operations_hash: value.operations_hash,
            fitness: value
                .fitness
                .into_iter()
                .map(|fitness| fitness.into())
                .collect(),
            context: value.context,
            payload_hash: Some(value.payload_hash),
            payload_round: value.payload_round,
            priority: 0,
            proof_of_work_nonce: value.proof_of_work_nonce.into(),
            seed_nonce_hash: value.seed_nonce_hash,
            liquidity_baking_escape_vote: Default::default(),
            liquidity_baking_toggle_vote: value.liquidity_baking_toggle_vote.into(),
            signature: Some(value.signature),
        }
    }
}

impl TryFrom<Header> for tezos_operation::block_header::BlockHeader {
    type Error = Error;

    fn try_from(value: Header) -> Result<Self> {
        Ok(Self {
            level: value.level,
            proto: value.proto,
            predecessor: value.predecessor,
            timestamp: value.timestamp,
            validation_pass: value.validation_pass,
            operations_hash: value.operations_hash,
            fitness: value
                .fitness
                .into_iter()
                .map(|fitness| Ok(fitness.try_into()?))
                .collect::<Result<Vec<_>>>()?,
            context: value.context,
            payload_hash: value.payload_hash.ok_or(Error::InvalidConversion)?,
            payload_round: value.payload_round,
            proof_of_work_nonce: value.proof_of_work_nonce.try_into()?,
            seed_nonce_hash: value.seed_nonce_hash,
            liquidity_baking_toggle_vote: value.liquidity_baking_toggle_vote.into(),
            signature: value.signature.ok_or(Error::InvalidConversion)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LiquidityBakingToggleVote {
    On,
    Off,
    Pass,
}

impl Default for LiquidityBakingToggleVote {
    fn default() -> Self {
        Self::Pass
    }
}

impl From<tezos_operation::block_header::LiquidityBakingToggleVote> for LiquidityBakingToggleVote {
    fn from(value: tezos_operation::block_header::LiquidityBakingToggleVote) -> Self {
        match value {
            tezos_operation::block_header::LiquidityBakingToggleVote::On => Self::On,
            tezos_operation::block_header::LiquidityBakingToggleVote::Off => Self::Off,
            tezos_operation::block_header::LiquidityBakingToggleVote::Pass => Self::Pass,
        }
    }
}

impl From<LiquidityBakingToggleVote> for tezos_operation::block_header::LiquidityBakingToggleVote {
    fn from(value: LiquidityBakingToggleVote) -> Self {
        match value {
            LiquidityBakingToggleVote::On => Self::On,
            LiquidityBakingToggleVote::Off => Self::Off,
            LiquidityBakingToggleVote::Pass => Self::Pass,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullHeader {
    pub protocol: ProtocolHash,
    pub chain_id: ChainId,
    pub hash: BlockHash,
    pub level: i32,
    pub proto: u8,
    pub predecessor: BlockHash,
    #[serde(with = "rfc3339_timestamp")]
    pub timestamp: NaiveDateTime,
    pub validation_pass: u8,
    pub operations_hash: OperationListListHash,
    pub fitness: Vec<String>,
    pub context: ContextHash,
    pub payload_hash: Option<BlockPayloadHash>,
    #[serde(default)]
    pub payload_round: i32,
    #[serde(default)]
    pub priority: u16,
    #[serde(default)]
    pub proof_of_work_nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_nonce_hash: Option<NonceHash>,
    #[serde(default)]
    pub liquidity_baking_escape_vote: bool,
    pub signature: Option<Signature>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Metadata {
    pub protocol: ProtocolHash,
    pub next_protocol: ProtocolHash,
    pub test_chain_status: TestChainStatus,
    pub max_operations_ttl: i32,
    pub max_operation_data_length: i32,
    pub max_block_header_length: i32,
    pub max_operation_list_length: Vec<OperationListLength>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baker: Option<ImplicitAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposer: Option<ImplicitAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_info: Option<LevelInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_period_kind: Option<VotingPeriodKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_period_info: Option<VotingPeriodInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce_hash: Option<NonceHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<Vec<ImplicitAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_baking_escape_ema: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_baking_toggle_ema: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_operations_results: Option<Vec<SuccessfulManagerOperationResult>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct OperationListLength {
    pub max_size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_op: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Level {
    pub level: i32,
    pub level_position: i32,
    pub cycle: i32,
    pub cycle_position: i32,
    pub voting_period: i32,
    pub voting_period_position: i32,
    pub expected_commitment: bool,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            level: 0,
            level_position: 0,
            cycle: 0,
            cycle_position: 0,
            voting_period: 0,
            voting_period_position: 0,
            expected_commitment: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct LevelInfo {
    pub level: i32,
    pub level_position: i32,
    pub cycle: i32,
    pub cycle_position: i32,
    pub expected_commitment: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VotingPeriodKind {
    Unknown,
    Proposal,
    TestingVote,
    Testing,
    PromotionVote,
    Adoption,
    Exploration,
    Cooldown,
    Promotion,
}

impl Default for VotingPeriodKind {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct VotingPeriodInfo {
    pub voting_period: VotingPeriod,
    pub position: i32,
    pub remaining: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct VotingPeriod {
    pub index: i32,
    pub kind: VotingPeriodKind,
    pub start_position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TestChainStatus {
    pub status: TestChainStatusName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genesis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<NaiveDateTime>,
}

/// The status of the test chain:
///
/// * `not_running` : There is no test chain at the moment.
/// * `forking` : The test chain is being setup.
/// * `running` : The test chain is running.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TestChainStatusName {
    NotRunning,
    Forking,
    Running,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Genesis,
    Hash(BlockHash),
    Level(i32),
}

impl Default for BlockId {
    fn default() -> Self {
        BlockId::Head
    }
}

impl BlockId {
    pub fn value(&self) -> String {
        match self {
            Self::Head => BLOCK_HEAD_ALIAS.into(),
            Self::Genesis => BLOCK_GENESIS_ALIAS.into(),
            Self::Hash(hash) => hash.value().into(),
            Self::Level(level) => {
                if level.is_negative() {
                    return format!("head~{}", level.abs());
                }
                format!("{}", level)
            }
        }
    }
}
