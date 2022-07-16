use tezos_core::types::encoded::Ed25519PublicKeyHash;

use {
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivateAccount {
    /// [OperationKind::ActivateAccount]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub pkh: Ed25519PublicKeyHash,
    /// /^([a-zA-Z0-9][a-zA-Z0-9])*$/
    pub secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
