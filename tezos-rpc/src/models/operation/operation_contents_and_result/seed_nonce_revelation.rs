use crate::{Error, Result};

use {
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeedNonceRevelation {
    /// [OperationKind::SeedNonceRevelation]
    pub kind: OperationKind,
    /// integer ∈ [-2^31-1, 2^31]
    pub level: i32,
    /// /^([a-zA-Z0-9][a-zA-Z0-9])*$/
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl From<tezos_operation::operations::SeedNonceRevelation> for SeedNonceRevelation {
    fn from(value: tezos_operation::operations::SeedNonceRevelation) -> Self {
        Self {
            kind: OperationKind::SeedNonceRevelation,
            level: value.level,
            nonce: value.nonce.into(),
            metadata: None,
        }
    }
}

impl TryFrom<SeedNonceRevelation> for tezos_operation::operations::SeedNonceRevelation {
    type Error = Error;

    fn try_from(value: SeedNonceRevelation) -> Result<Self> {
        Ok(Self {
            level: value.level,
            nonce: value.nonce.try_into()?,
        })
    }
}
