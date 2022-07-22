use tezos_core::types::encoded::{BlockHash, Signature};

use crate::{Error, Result};

use {
    super::preendorsement::Preendorsement,
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoublePreendorsementEvidence {
    /// [OperationKind::DoublePreendorsementEvidence]
    pub kind: OperationKind,
    pub op1: InlinedPreendorsement,
    pub op2: InlinedPreendorsement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl From<tezos_operation::operations::DoublePreendorsementEvidence>
    for DoublePreendorsementEvidence
{
    fn from(value: tezos_operation::operations::DoublePreendorsementEvidence) -> Self {
        Self {
            kind: OperationKind::DoublePreendorsementEvidence,
            op1: value.op1.into(),
            op2: value.op2.into(),
            metadata: None,
        }
    }
}

impl TryFrom<DoublePreendorsementEvidence>
    for tezos_operation::operations::DoublePreendorsementEvidence
{
    type Error = Error;

    fn try_from(value: DoublePreendorsementEvidence) -> Result<Self> {
        Ok(Self {
            op1: value.op1.try_into()?,
            op2: value.op2.try_into()?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InlinedPreendorsement {
    /// A block identifier (Base58Check-encoded)
    pub branch: BlockHash,
    pub operations: Preendorsement,
    /// A Ed25519, Secp256k1 or P256 signature (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
}

impl From<tezos_operation::operations::InlinedPreendrosement> for InlinedPreendorsement {
    fn from(value: tezos_operation::operations::InlinedPreendrosement) -> Self {
        Self {
            branch: value.branch,
            operations: value.operations.into(),
            signature: Some(value.signature),
        }
    }
}

impl TryFrom<InlinedPreendorsement> for tezos_operation::operations::InlinedPreendrosement {
    type Error = Error;

    fn try_from(value: InlinedPreendorsement) -> Result<Self> {
        Ok(Self {
            branch: value.branch,
            operations: value.operations.into(),
            signature: value.signature.ok_or(Error::InvalidConversion)?,
        })
    }
}
