use {
    crate::models::operation::kind::OperationKind,
    crate::models::operation::metadata::Metadata,
    crate::{Error, Result},
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::Ed25519PublicKeyHash,
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

impl From<tezos_operation::operations::ActivateAccount> for ActivateAccount {
    fn from(value: tezos_operation::operations::ActivateAccount) -> Self {
        Self {
            kind: OperationKind::ActivateAccount,
            pkh: value.pkh,
            secret: value.secret.into_string(false),
            metadata: None,
        }
    }
}

impl TryFrom<ActivateAccount> for tezos_operation::operations::ActivateAccount {
    type Error = Error;

    fn try_from(value: ActivateAccount) -> Result<Self> {
        Ok(Self {
            pkh: value.pkh,
            secret: value.secret.try_into()?,
        })
    }
}
