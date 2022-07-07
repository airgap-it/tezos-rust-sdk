use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone)]
pub struct SetDepositsLimit {
    source: ImplicitAddress,
    fee: Mutez,
    counter: Nat,
    gas_limit: Nat,
    storage_limit: Nat,
    limit: Mutez,
}

impl SetDepositsLimit {
    pub fn limit(&self) -> Mutez {
        self.limit
    }

    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        limit: Mutez,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            limit,
        }
    }
}

impl TraitOperationContent for SetDepositsLimit {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::SetDepositsLimit as u8]
    }
}

impl TraitOperationManagerContent for SetDepositsLimit {
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
