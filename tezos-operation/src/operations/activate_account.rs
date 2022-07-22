use tezos_core::types::{encoded::Ed25519PublicKeyHash, hex_string::HexString};

use super::{OperationContentTag, TraitOperationContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateAccount {
    pub pkh: Ed25519PublicKeyHash,
    pub secret: HexString,
}

impl ActivateAccount {
    pub fn new(pkh: Ed25519PublicKeyHash, secret: HexString) -> Self {
        Self { pkh, secret }
    }
}

impl TraitOperationContent for ActivateAccount {
    fn tag() -> OperationContentTag {
        OperationContentTag::ActivateAccount
    }
}
