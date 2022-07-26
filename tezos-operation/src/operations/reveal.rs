use tezos_core::types::{
    encoded::{ImplicitAddress, PublicKey},
    mutez::Mutez,
    number::Nat,
};

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reveal {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub public_key: PublicKey,
}

impl Reveal {
    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        public_key: PublicKey,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            public_key,
        }
    }
}

impl TraitOperationContent for Reveal {
    fn tag() -> OperationContentTag {
        OperationContentTag::Reveal
    }
}

impl TraitOperationManagerContent for Reveal {
    fn source(&self) -> &ImplicitAddress {
        &self.source
    }

    fn fee(&self) -> Mutez {
        self.fee
    }

    fn counter(&self) -> &Nat {
        &self.counter
    }

    fn gas_limit(&self) -> &Nat {
        &self.gas_limit
    }

    fn storage_limit(&self) -> &Nat {
        &self.storage_limit
    }
}
