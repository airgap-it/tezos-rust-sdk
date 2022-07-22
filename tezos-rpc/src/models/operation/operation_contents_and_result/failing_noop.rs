use crate::{Error, Result};

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

impl From<tezos_operation::operations::FailingNoop> for FailingNoop {
    fn from(value: tezos_operation::operations::FailingNoop) -> Self {
        Self {
            kind: OperationKind::FailingNoop,
            arbitrary: value.arbitrary.into(),
        }
    }
}

impl TryFrom<FailingNoop> for tezos_operation::operations::FailingNoop {
    type Error = Error;

    fn try_from(value: FailingNoop) -> Result<Self> {
        Ok(Self {
            arbitrary: value.arbitrary.try_into()?,
        })
    }
}
