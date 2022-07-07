use super::{InlinedEndorsement, OperationContentTag, TraitOperationContent};

pub struct DoubleEndorsementEvidence {
    op1: InlinedEndorsement,
    op2: InlinedEndorsement,
}

impl DoubleEndorsementEvidence {
    pub fn op1(&self) -> &InlinedEndorsement {
        &self.op1
    }

    pub fn op2(&self) -> &InlinedEndorsement {
        &self.op2
    }

    pub fn new(op1: InlinedEndorsement, op2: InlinedEndorsement) -> Self {
        Self { op1, op2 }
    }
}

impl TraitOperationContent for DoubleEndorsementEvidence {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::DoubleEndorsementEvidence as u8]
    }
}
