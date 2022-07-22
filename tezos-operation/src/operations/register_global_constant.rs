use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterGlobalConstant {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub value: Micheline,
}

impl RegisterGlobalConstant {
    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        value: Micheline,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            value,
        }
    }
}

impl TraitOperationContent for RegisterGlobalConstant {
    fn tag() -> OperationContentTag {
        OperationContentTag::RegisterGlobalConstant
    }
}

impl TraitOperationManagerContent for RegisterGlobalConstant {
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
