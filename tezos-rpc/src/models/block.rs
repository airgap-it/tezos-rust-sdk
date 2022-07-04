use {
    super::balance_update::BalanceUpdate,
    super::operation::OperationGroup,
    super::operation::operation_result::operations::SuccessfulManagerOperationResult,
    tezos_core::helper::rfc3339_timestamp,
    tezos_core::types::timestamp::Timestamp,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub protocol: String,
    pub chain_id: String,
    pub hash: String,
    pub header: Header,
    pub operations: Vec<Vec<OperationGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Header {
    pub level: i32,
    pub proto: u8,
    pub predecessor: String,
    #[serde(with = "rfc3339_timestamp")]
    pub timestamp: Timestamp,
    pub validation_pass: u8,
    pub operations_hash: String,
    pub fitness: Vec<String>,
    pub context: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub payload_hash: String,
    #[serde(default)]
    pub payload_round: i32,
    #[serde(default)]
    pub priority: u16,
    #[serde(default)]
    pub proof_of_work_nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_nonce_hash: Option<String>,
    #[serde(default)]
    pub liquidity_baking_escape_vote: bool,
    #[serde(default)]
    pub signature: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullHeader {
    pub protocol: String,
    pub chain_id: String,
    pub hash: String,
    pub level: i32,
    pub proto: u8,
    pub predecessor: String,
    #[serde(with = "rfc3339_timestamp")]
    pub timestamp: Timestamp,
    pub validation_pass: u8,
    pub operations_hash: String,
    pub fitness: Vec<String>,
    pub context: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub payload_hash: String,
    #[serde(default)]
    pub payload_round: i32,
    #[serde(default)]
    pub priority: u16,
    #[serde(default)]
    pub proof_of_work_nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_nonce_hash: Option<String>,
    #[serde(default)]
    pub liquidity_baking_escape_vote: bool,
    #[serde(default)]
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Metadata {
    pub protocol: String,
    pub next_protocol: String,
    pub max_operations_ttl: i32,
    pub max_operation_data_length: i32,
    pub max_block_header_length: i32,
    pub max_operation_list_length: Vec<OperationListLength>,
    #[serde(default)]
    pub baker: String,
    #[serde(default)]
    pub proposer: String,
    #[serde(default)]
    pub level: Level,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_info: Option<LevelInfo>,
    #[serde(default)]
    pub voting_period_kind: VotingPeriodKind,
    pub voting_period_info: Option<VotingPeriodInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce_hash: Option<String>,
    #[serde(default)]
    pub consumed_gas: String,
    #[serde(default)]
    pub deactivated: Vec<String>,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
    /// integer ∈ [-2^31-1, 2^31],
    pub liquidity_baking_escape_ema: Option<u32>,
    #[serde(default)]
    pub implicit_operations_results: Option<Vec<SuccessfulManagerOperationResult>>
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
