use crate::block_header::BlockHeader;

use super::{OperationContentTag, TraitOperationContent};

pub struct DoubleBakingEvidence {
    bh1: BlockHeader,
    bh2: BlockHeader,
}

impl DoubleBakingEvidence {
    pub fn bh1(&self) -> &BlockHeader {
        &self.bh1
    }

    pub fn bh2(&self) -> &BlockHeader {
        &self.bh2
    }

    pub fn new(bh1: BlockHeader, bh2: BlockHeader) -> Self {
        Self { bh1, bh2 }
    }
}

impl TraitOperationContent for DoubleBakingEvidence {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::DoubleBakingEvidence as u8]
    }
}
