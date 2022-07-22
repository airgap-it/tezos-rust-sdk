use crate::block_header::BlockHeader;

use super::{OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleBakingEvidence {
    pub bh1: BlockHeader,
    pub bh2: BlockHeader,
}

impl DoubleBakingEvidence {
    pub fn new(bh1: BlockHeader, bh2: BlockHeader) -> Self {
        Self { bh1, bh2 }
    }
}

impl TraitOperationContent for DoubleBakingEvidence {
    fn tag() -> OperationContentTag {
        OperationContentTag::DoubleBakingEvidence
    }
}
