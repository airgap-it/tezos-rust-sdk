use tezos_core::types::encoded::{BlockHash, Signature};

use crate::{Error, Result};

use {
    super::endorsement::Endorsement,
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoubleEndorsementEvidence {
    /// [OperationKind::DoubleEndorsementEvidence]
    pub kind: OperationKind,
    pub op1: InlinedEndorsement,
    pub op2: InlinedEndorsement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl From<tezos_operation::operations::DoubleEndorsementEvidence> for DoubleEndorsementEvidence {
    fn from(value: tezos_operation::operations::DoubleEndorsementEvidence) -> Self {
        Self {
            kind: OperationKind::DoubleEndorsementEvidence,
            op1: value.op1.into(),
            op2: value.op2.into(),
            metadata: None,
        }
    }
}

impl TryFrom<DoubleEndorsementEvidence> for tezos_operation::operations::DoubleEndorsementEvidence {
    type Error = Error;

    fn try_from(value: DoubleEndorsementEvidence) -> Result<Self> {
        Ok(Self {
            op1: value.op1.try_into()?,
            op2: value.op2.try_into()?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlinedEndorsement {
    /// A block identifier (Base58Check-encoded)
    pub branch: BlockHash,
    pub operations: Endorsement,
    /// A Ed25519, Secp256k1 or P256 signature (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
}

impl From<tezos_operation::operations::InlinedEndorsement> for InlinedEndorsement {
    fn from(value: tezos_operation::operations::InlinedEndorsement) -> Self {
        Self {
            branch: value.branch,
            operations: value.operations.into(),
            signature: Some(value.signature),
        }
    }
}

impl TryFrom<InlinedEndorsement> for tezos_operation::operations::InlinedEndorsement {
    type Error = Error;

    fn try_from(value: InlinedEndorsement) -> Result<Self> {
        Ok(Self {
            branch: value.branch,
            operations: value.operations.try_into()?,
            signature: value.signature.ok_or(Error::InvalidConversion)?,
        })
    }
}
