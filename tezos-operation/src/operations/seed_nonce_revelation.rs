use tezos_core::types::hex_string::HexString;

use super::{OperationContentTag, TraitOperationContent};

pub struct SeedNonceRevelation {
    level: i32,
    nonce: HexString,
}

impl SeedNonceRevelation {
    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn nonce(&self) -> &HexString {
        &self.nonce
    }

    pub fn new(level: i32, nonce: HexString) -> Self {
        Self { level, nonce }
    }
}

impl TraitOperationContent for SeedNonceRevelation {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::SeedNonceRevelation as u8]
    }
}
