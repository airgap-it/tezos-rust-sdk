use tezos_core::types::{
    encoded::{Address, ImplicitAddress},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: Nat,
    pub gas_limit: Nat,
    pub storage_limit: Nat,
    pub amount: Mutez,
    pub destination: Address,
    pub parameters: Option<Parameters>,
}

impl Transaction {
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
    pub entrypoint: Entrypoint,
    pub value: Micheline,
}

impl Parameters {
    pub fn new(entrypoint: Entrypoint, value: Micheline) -> Self {
        Self { entrypoint, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Entrypoint {
    Default,
    Root,
    Do,
    SetDelegate,
    RemoveDelegate,
    Named(String),
}

impl Entrypoint {
    const DEFAULT_TAG: u8 = 0;
    const ROOT_TAG: u8 = 1;
    const DO_TAG: u8 = 2;
    const SET_DELEGATE_TAG: u8 = 3;
    const REMOVE_DELEGATE_TAG: u8 = 4;
    const NAMED_TAG: u8 = 255;

    const DEFAULT: &'static str = "default";
    const ROOT: &'static str = "root";
    const DO: &'static str = "do";
    const SET_DELEGATE: &'static str = "set_delegate";
    const REMOVE_DELEGATE: &'static str = "remove_delegate";

    pub fn tag(&self) -> u8 {
        match self {
            Self::Default => Self::DEFAULT_TAG,
            Self::Root => Self::ROOT_TAG,
            Self::Do => Self::DO_TAG,
            Self::SetDelegate => Self::SET_DELEGATE_TAG,
            Self::RemoveDelegate => Self::REMOVE_DELEGATE_TAG,
            Self::Named(_) => Self::named_tag(),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Default => Self::DEFAULT,
            Self::Root => Self::ROOT,
            Self::Do => Self::DO,
            Self::SetDelegate => Self::SET_DELEGATE,
            Self::RemoveDelegate => Self::REMOVE_DELEGATE,
            Self::Named(value) => value.as_str(),
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            Self::DEFAULT => Self::Default,
            Self::ROOT => Self::Root,
            Self::DO => Self::Do,
            Self::SET_DELEGATE => Self::SetDelegate,
            Self::REMOVE_DELEGATE => Self::RemoveDelegate,
            _ => Self::Named(value.into()),
        }
    }

    pub fn named_tag() -> u8 {
        Self::NAMED_TAG
    }

    pub fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            Self::DEFAULT_TAG => Some(Self::Default),
            Self::ROOT_TAG => Some(Self::Root),
            Self::DO_TAG => Some(Self::Do),
            Self::SET_DELEGATE_TAG => Some(Self::SetDelegate),
            Self::REMOVE_DELEGATE_TAG => Some(Self::RemoveDelegate),
            _ => None,
        }
    }
}

impl Default for Entrypoint {
    fn default() -> Self {
        Self::Default
    }
}

impl From<&str> for Entrypoint {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}
