use tezos_core::types::hex_string::HexString;

use super::{OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailingNoop {
    arbitrary: HexString,
}

impl FailingNoop {
    pub fn arbitrary(&self) -> &HexString {
        &self.arbitrary
    }

    pub fn new(arbitrary: HexString) -> Self {
        Self { arbitrary }
    }
}

impl TraitOperationContent for FailingNoop {
    fn tag() -> OperationContentTag {
        OperationContentTag::FailingNoop
    }
}
