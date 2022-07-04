use {
    crate::models::operation::kind::OperationKind,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailingNoop {
    /// [OperationKind::FailingNoop]
    pub kind: OperationKind,
    /// Arbitrary string
    pub arbitrary: String,
}
