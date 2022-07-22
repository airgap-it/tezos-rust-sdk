use tezos_core::types::hex_string::HexString;

use super::{OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedNonceRevelation {
    pub level: i32,
    pub nonce: HexString,
}

impl SeedNonceRevelation {
    pub fn new(level: i32, nonce: HexString) -> Self {
        Self { level, nonce }
    }
}

impl TraitOperationContent for SeedNonceRevelation {
    fn tag() -> OperationContentTag {
        OperationContentTag::SeedNonceRevelation
    }
}
