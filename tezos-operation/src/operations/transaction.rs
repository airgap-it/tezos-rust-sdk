use num_derive::FromPrimitive;
use tezos_core::types::{
    encoded::{Address, ImplicitAddress},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    source: ImplicitAddress,
    fee: Mutez,
    counter: Nat,
    gas_limit: Nat,
    storage_limit: Nat,
    amount: Mutez,
    destination: Address,
    parameters: Option<Parameters>,
}

impl Transaction {
    pub fn amount(&self) -> Mutez {
        self.amount
    }

    pub fn destination(&self) -> &Address {
        &self.destination
    }

    pub fn parameters(&self) -> Option<&Parameters> {
        self.parameters.as_ref()
    }

    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        amount: Mutez,
        destination: Address,
        parameters: Option<Parameters>,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            amount,
            destination,
            parameters,
        }
    }
}

impl TraitOperationContent for Transaction {
    fn tag() -> OperationContentTag {
        OperationContentTag::Transaction
    }
}

impl TraitOperationManagerContent for Transaction {
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
pub struct Parameters {
    entrypoint: Entrypoint,
    value: Micheline,
}

impl Parameters {
    pub fn entrypoint(&self) -> &Entrypoint {
        &self.entrypoint
    }

    pub fn value(&self) -> &Micheline {
        &self.value
    }

    pub fn new(entrypoint: Entrypoint, value: Micheline) -> Self {
        Self { entrypoint, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Entrypoint {
    Primitive(PrimitiveEntrypoint),
    Named(String),
}

impl Entrypoint {
    const NAMED_TAG: u8 = 255;

    pub fn tag(&self) -> u8 {
        match self {
            Entrypoint::Primitive(value) => value.tag(),
            Entrypoint::Named(_) => Self::named_tag(),
        }
    }

    pub fn named_tag() -> u8 {
        Self::NAMED_TAG
    }
}

impl Entrypoint {
    fn default() -> Self {
        Self::Primitive(PrimitiveEntrypoint::Default)
    }

    pub fn root() -> Self {
        Self::Primitive(PrimitiveEntrypoint::Root)
    }

    pub fn r#do() -> Self {
        Self::Primitive(PrimitiveEntrypoint::Do)
    }

    pub fn set_delegate() -> Self {
        Self::Primitive(PrimitiveEntrypoint::SetDelegate)
    }

    pub fn remove_delegate() -> Self {
        Self::Primitive(PrimitiveEntrypoint::RemoveDelegate)
    }

    pub fn named(name: String) -> Self {
        Self::Named(name)
    }
}

impl Default for Entrypoint {
    fn default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum PrimitiveEntrypoint {
    Default = 0,
    Root = 1,
    Do = 2,
    SetDelegate = 3,
    RemoveDelegate = 4,
}

impl PrimitiveEntrypoint {
    const DEFAULT: &'static str = "default";
    const ROOT: &'static str = "root";
    const DO: &'static str = "do";
    const SET_DELEGATE: &'static str = "set_delegate";
    const REMOVE_DELEGATE: &'static str = "remove_delegate";

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Default => Self::DEFAULT,
            Self::Root => Self::ROOT,
            Self::Do => Self::DO,
            Self::SetDelegate => Self::SET_DELEGATE,
            Self::RemoveDelegate => Self::REMOVE_DELEGATE,
        }
    }

    pub fn tag(&self) -> u8 {
        *self as u8
    }
}
