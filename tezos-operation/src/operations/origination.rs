use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};
use tezos_michelson::micheline::{sequence::Sequence, Micheline};

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origination {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub balance: Mutez,
    pub delegate: Option<ImplicitAddress>,
    pub script: Script,
}

impl Origination {
    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        balance: Mutez,
        delegate: Option<ImplicitAddress>,
        script: Script,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            balance,
            delegate,
            script,
        }
    }
}

impl TraitOperationContent for Origination {
    fn tag() -> OperationContentTag {
        OperationContentTag::Origination
    }
}

impl TraitOperationManagerContent for Origination {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub code: Sequence,
    pub storage: Micheline,
}

impl Script {
    pub fn new(code: Sequence, storage: Micheline) -> Self {
        Self { code, storage }
    }
}
