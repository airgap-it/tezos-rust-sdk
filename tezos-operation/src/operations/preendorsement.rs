use tezos_core::types::encoded::BlockPayloadHash;

use super::{OperationContentTag, TraitOperationConsensusContent, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preendorsement {
    pub slot: u16,
    pub level: i32,
    pub round: i32,
    pub block_payload_hash: BlockPayloadHash,
}

impl Preendorsement {
    pub fn new(slot: u16, level: i32, round: i32, block_payload_hash: BlockPayloadHash) -> Self {
        Self {
            slot,
            level,
            round,
            block_payload_hash,
        }
    }
}

impl TraitOperationContent for Preendorsement {
    fn tag() -> OperationContentTag {
        OperationContentTag::Preendorsement
    }
}

impl TraitOperationConsensusContent for Preendorsement {
    fn slot(&self) -> u16 {
        self.slot
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn round(&self) -> i32 {
        self.round
    }

    fn block_payload_hash(&self) -> &BlockPayloadHash {
        &self.block_payload_hash
    }
}
