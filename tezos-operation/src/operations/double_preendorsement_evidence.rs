use super::{InlinedPreendrosement, OperationContentTag, TraitOperationContent};

pub struct DoublePreendorsementEvidence {
    op1: InlinedPreendrosement,
    op2: InlinedPreendrosement,
}

impl DoublePreendorsementEvidence {
    pub fn op1(&self) -> &InlinedPreendrosement {
        &self.op1
    }

    pub fn op2(&self) -> &InlinedPreendrosement {
        &self.op2
    }

    pub fn new(op1: InlinedPreendrosement, op2: InlinedPreendrosement) -> Self {
        Self { op1, op2 }
    }
}

impl TraitOperationContent for DoublePreendorsementEvidence {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::DoublePreendorsementEvidence as u8]
    }
}
