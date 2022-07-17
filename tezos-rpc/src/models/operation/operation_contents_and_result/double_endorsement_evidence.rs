use tezos_core::types::encoded::{BlockHash, Signature};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlinedEndorsement {
    /// A block identifier (Base58Check-encoded)
    pub branch: BlockHash,
    pub operations: Endorsement,
    /// A Ed25519, Secp256k1 or P256 signature (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
}
