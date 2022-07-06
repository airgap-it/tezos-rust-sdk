use tezos_core::types::encoded::BlockPayloadHash;

use super::{OperationContentTag, TraitOperationContent};

pub struct Endorsement {
    slot: u8,
    level: i32,
    round: i32,
    block_payload_hash: BlockPayloadHash,
}

impl Endorsement {
    pub fn slot(&self) -> u8 {
        self.slot
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn round(&self) -> i32 {
        self.round
    }

    pub fn block_payload_hash(&self) -> &BlockPayloadHash {
        &self.block_payload_hash
    }

    pub fn new(slot: u8, level: i32, round: i32, block_payload_hash: BlockPayloadHash) -> Self {
        Self {
            slot,
            level,
            round,
            block_payload_hash,
        }
    }
}

impl TraitOperationContent for Endorsement {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::Endorsement as u8]
    }
}
