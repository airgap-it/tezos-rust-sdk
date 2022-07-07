use {
    crate::models::block::FullHeader,
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoubleBakingEvidence {
    /// [OperationKind::DoubleBakingEvidence]
    pub kind: OperationKind,
    pub bh1: FullHeader,
    pub bh2: FullHeader,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
