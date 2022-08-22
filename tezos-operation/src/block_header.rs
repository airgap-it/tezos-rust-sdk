use chrono::NaiveDateTime;
use num_derive::{FromPrimitive, ToPrimitive};
use tezos_core::types::{
    encoded::{
        BlockHash, BlockPayloadHash, ContextHash, NonceHash, OperationListListHash, Signature,
    },
    hex_string::HexString,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub level: i32,
    pub proto: u8,
    pub predecessor: BlockHash,
    pub timestamp: NaiveDateTime,
    pub validation_pass: u8,
    pub operations_hash: OperationListListHash,
    pub fitness: Vec<HexString>,
    pub context: ContextHash,
    pub payload_hash: BlockPayloadHash,
    pub payload_round: i32,
    pub proof_of_work_nonce: HexString,
    pub seed_nonce_hash: Option<NonceHash>,
    pub liquidity_baking_toggle_vote: LiquidityBakingToggleVote,
    pub signature: Signature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum LiquidityBakingToggleVote {
    On = 0,
    Off = 1,
    Pass = 2,
}
