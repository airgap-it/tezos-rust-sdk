#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tezos_core::types::number::Nat;

pub use crate::common::{bytes::Bytes, string::String};
use crate::{Error, Result};
use alloc::format;
use alloc::string::ToString;
pub use tezos_core::types::number::Int;

use super::Micheline;

/// `Micheline` literals as defined in [the documentation](https://tezos.gitlab.io/shell/micheline.html#bnf-grammar).
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Literal {
    Int(Int),
    String(String),
    Bytes(Bytes),
}

impl Literal {
    pub fn is_int(&self) -> bool {
        if let Literal::Int(_) = self {
            return true;
        }
        return false;
    }

    pub fn is_string(&self) -> bool {
        if let Literal::String(_) = self {
            return true;
        }
        return false;
    }

    pub fn is_bytes(&self) -> bool {
        if let Literal::Bytes(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_micheline_int(self) -> Option<Int> {
        if let Literal::Int(value) = self {
            return Some(value);
        }
        return None;
    }

    pub fn into_micheline_string(self) -> Option<String> {
        if let Literal::String(value) = self {
            return Some(value);
        }
        return None;
    }

    pub fn into_micheline_bytes(self) -> Option<Bytes> {
        if let Literal::Bytes(value) = self {
            return Some(value);
        }
        return None;
    }
}

impl TryFrom<Micheline> for Literal {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        if let Micheline::Literal(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMicheline {
            description: format!("Cannot convert {:?} to a Micheline Literal", value),
        })
    }
}

impl From<Int> for Literal {
    fn from(value: Int) -> Self {
        Literal::Int(value)
    }
}

impl From<&Int> for Literal {
    fn from(value: &Int) -> Self {
        Literal::Int(value.clone())
    }
}

impl From<Int> for Micheline {
    fn from(value: Int) -> Self {
        let literal: Literal = value.into();
        literal.into()
    }
}

impl From<&Int> for Micheline {
    fn from(value: &Int) -> Self {
        let literal: Literal = value.into();
        literal.into()
    }
}

impl From<Nat> for Micheline {
    fn from(value: Nat) -> Self {
        Literal::Int(value.into()).into()
    }
}

impl From<&Nat> for Micheline {
    fn from(value: &Nat) -> Self {
        Literal::Int(value.into()).into()
    }
}

impl TryFrom<Micheline> for Nat {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        if let Micheline::Literal(Literal::Int(value)) = value {
            return Ok(value.to_string().try_into()?);
        }
        Err(Error::InvalidMicheline {
            description: format!("Cannot convert {:?} to a Nat", value),
        })
    }
}

impl TryFrom<Literal> for Int {
    type Error = Error;

    fn try_from(value: Literal) -> Result<Self> {
        if let Literal::Int(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelineLiteral)
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String(value)
    }
}

impl From<&String> for Literal {
    fn from(value: &String) -> Self {
        Literal::String(value.clone())
    }
}

impl From<String> for Micheline {
    fn from(value: String) -> Self {
        let literal: Literal = value.into();
        literal.into()
    }
}

impl From<&String> for Micheline {
    fn from(value: &String) -> Self {
        let literal: Literal = value.into();
        literal.into()
    }
}

impl TryFrom<alloc::string::String> for Literal {
    type Error = Error;

    fn try_from(value: alloc::string::String) -> Result<Self> {
        let string: String = value.try_into()?;

        Ok(Literal::String(string))
    }
}

impl From<Bytes> for Literal {
    fn from(value: Bytes) -> Self {
        Literal::Bytes(value)
    }
}

impl From<&Bytes> for Literal {
    fn from(value: &Bytes) -> Self {
        Literal::Bytes(value.clone())
    }
}

impl From<Bytes> for Micheline {
    fn from(value: Bytes) -> Self {
        let literal: Literal = value.into();
        literal.into()
    }
}

impl From<&Bytes> for Micheline {
    fn from(value: &Bytes) -> Self {
        let literal: Literal = value.into();
        literal.into()
    }
}
