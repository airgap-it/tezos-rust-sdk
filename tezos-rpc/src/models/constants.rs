use {
    crate::serde_utils,
    num_bigint::BigInt,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Constants {
    /// integer ∈ [0, 255]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_work_nonce_size: Option<u8>,
    /// integer ∈ [0, 255]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce_length: Option<u8>,
    /// integer ∈ [0, 255]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_anon_ops_per_block: Option<u8>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_operation_data_length: Option<i32>,
    /// integer ∈ [0, 255]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_proposals_per_delegate: Option<u8>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_micheline_node_count: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_micheline_bytes_limit: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allowed_global_constants_depth: Option<i32>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_vec_of_option_string_vec"
    )]
    pub cache_layout: Option<Vec<i64>>,
    /// integer ∈ [0, 2^16-1]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub michelson_maximum_type_size: Option<u16>,
    /// integer ∈ [0, 255]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserved_cycles: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks_per_cycle: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks_per_commitment: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks_per_stake_snapshot: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks_per_voting_period: Option<i32>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub hard_gas_limit_per_operation: Option<BigInt>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub hard_gas_limit_per_block: Option<BigInt>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub proof_of_work_threshold: Option<i64>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub seed_nonce_revelation_tip: Option<BigInt>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_size: Option<i32>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub baking_reward_fixed_portion: Option<BigInt>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub baking_reward_bonus_per_slot: Option<BigInt>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub endorsing_reward_per_slot: Option<BigInt>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub cost_per_byte: Option<BigInt>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub hard_storage_limit_per_operation: Option<BigInt>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_min: Option<i32>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_max: Option<i32>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_proposal_quorum: Option<i32>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub liquidity_baking_subsidy: Option<i64>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_baking_sunset_level: Option<i32>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_baking_toggle_ema_threshold: Option<i32>,
    /// integer ∈ [-2^15, 2^15-1]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_operations_time_to_live: Option<i16>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub minimal_block_delay: Option<i64>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub delay_increment_per_round: Option<i64>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consensus_committee_size: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consensus_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_participation_ratio: Option<RatioConstant>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slashing_period: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_deposits_percentage: Option<i32>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub double_baking_punishment: Option<BigInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio_of_frozen_deposits_slashed_per_double_endorsement: Option<RatioConstant>,
    /// A random generation state (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_seed: Option<String>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_script_size: Option<i32>,
    /// integer ∈ [-128, 127]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_stake_distribution_cycles: Option<u8>,
    /// integer ∈ [-128, 127]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_sampler_state_cycles: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_enable: Option<bool>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_origination_size: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_hard_size_limit_per_inbox: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_hard_size_limit_per_message: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_max_withdrawals_per_batch: Option<i32>,
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub tx_rollup_commitment_bond: Option<BigInt>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_finality_period: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_withdraw_period: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_max_inboxes_count: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_max_messages_per_inbox: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_max_commitments_count: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_cost_per_byte_ema_factor: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_max_ticket_payload_size: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_rejection_max_proof_size: Option<i32>,
    /// integer ∈ [-2^31-1, 2^31]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rollup_sunset_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sc_rollup_enable: Option<bool>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sc_rollup_origination_size: Option<i32>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sc_rollup_challenge_window_in_blocks: Option<i32>,

    // Added in Lima protocol
    /// Mutez
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub minimal_stake: Option<BigInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_rollup_enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_rollup_origination_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_rollup_min_pending_to_process: Option<i32>,

    // ===========================
    // Removed in recent protocols
    // ===========================
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    block_security_deposit: Option<u64>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    endorsement_security_deposit: Option<u64>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    block_reward: Option<u64>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    endorsement_reward: Option<u64>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    origination_burn: Option<u64>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    endorsers_per_block: Option<u16>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    blocks_per_roll_snapshot: Option<u16>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub max_revelations_per_block: Option<u16>,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_vec_of_option_string_vec"
    )]
    pub time_between_blocks: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_baking_escape_ema_threshold: Option<i32>,
    // Removed in Lima protocol
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub tokens_per_roll: Option<BigInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sc_rollup_max_available_messages: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatioConstant {
    /// integer ∈ [0, 2^16-1]
    numerator: u16,
    /// integer ∈ [0, 2^16-1]
    denominator: u16,
}
