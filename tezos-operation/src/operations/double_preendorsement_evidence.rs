use super::{InlinedPreendrosement, OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoublePreendorsementEvidence {
    pub op1: InlinedPreendrosement,
    pub op2: InlinedPreendrosement,
}

impl DoublePreendorsementEvidence {
    pub fn new(op1: InlinedPreendrosement, op2: InlinedPreendrosement) -> Self {
        Self { op1, op2 }
    }
}

impl TraitOperationContent for DoublePreendorsementEvidence {
    fn tag() -> OperationContentTag {
        OperationContentTag::DoublePreendorsementEvidence
    }
}
