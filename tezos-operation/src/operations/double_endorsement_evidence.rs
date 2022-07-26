use super::{InlinedEndorsement, OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleEndorsementEvidence {
    pub op1: InlinedEndorsement,
    pub op2: InlinedEndorsement,
}

impl DoubleEndorsementEvidence {
    pub fn new(op1: InlinedEndorsement, op2: InlinedEndorsement) -> Self {
        Self { op1, op2 }
    }
}

impl TraitOperationContent for DoubleEndorsementEvidence {
    fn tag() -> OperationContentTag {
        OperationContentTag::DoubleEndorsementEvidence
    }
}
