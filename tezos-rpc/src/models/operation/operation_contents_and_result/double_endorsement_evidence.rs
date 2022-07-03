use {
    super::endorsement::Endorsement,
    crate::models::operation::kind::Kind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoubleEndorsementEvidence {
    /// [Kind::DoubleEndorsementEvidence]
    pub kind: Kind,
    pub op1: InlinedEndorsement,
    pub op2: InlinedEndorsement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InlinedEndorsement {
    /// A block identifier (Base58Check-encoded)
    pub branch: String,
    pub operations: Endorsement,
    /// A Ed25519, Secp256k1 or P256 signature (Base58Check-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}
