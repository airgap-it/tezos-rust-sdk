use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone)]
pub struct RegisterGlobalConstant {
    source: ImplicitAddress,
    fee: Mutez,
    counter: Nat,
    gas_limit: Nat,
    storage_limit: Nat,
    value: Micheline,
}

impl RegisterGlobalConstant {
    pub fn value(&self) -> &Micheline {
        &self.value
    }

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
    fn tag() -> &'static [u8] {
        &[OperationContentTag::RegisterGlobalConstant as u8]
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
