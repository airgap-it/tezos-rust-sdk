use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDepositsLimit {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub limit: Option<Mutez>,
}

impl SetDepositsLimit {
    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        limit: Option<Mutez>,
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
    fn tag() -> OperationContentTag {
        OperationContentTag::SetDepositsLimit
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
