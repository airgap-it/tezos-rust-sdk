use crate::{Error, Result};

use {
    crate::models::block::Header,
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoubleBakingEvidence {
    /// [OperationKind::DoubleBakingEvidence]
    pub kind: OperationKind,
    pub bh1: Header,
    pub bh2: Header,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl From<tezos_operation::operations::DoubleBakingEvidence> for DoubleBakingEvidence {
    fn from(value: tezos_operation::operations::DoubleBakingEvidence) -> Self {
        Self {
            kind: OperationKind::DoubleBakingEvidence,
            bh1: value.bh1.into(),
            bh2: value.bh2.into(),
            metadata: None,
        }
    }
}

impl TryFrom<DoubleBakingEvidence> for tezos_operation::operations::DoubleBakingEvidence {
    type Error = Error;

    fn try_from(value: DoubleBakingEvidence) -> Result<Self> {
        Ok(Self {
            bh1: value.bh1.try_into()?,
            bh2: value.bh2.try_into()?,
        })
    }
}
