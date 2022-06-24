use crate::{michelson::Michelson, Error, Result};

use super::macros::{make_type, make_types};

make_types!(
    [
        fn fallback(_value: PrimitiveApplication) -> Result<Self> {
            Err(Error::InvalidPrimitiveApplication)
        }
    ],
    conversion_fallback: fallback,
    (Unit, unit, 108),
    (Never, never, 120),
    (Bool, bool, 89),
    (Int, int, 91),
    (Nat, nat, 98),
    (String, string, 104),
    (ChainId, chain_id, 116),
    (Bytes, bytes, 105),
    (Mutez, mutez, 106),
    (KeyHash, key_hash, 93),
    (Key, key, 92),
    (Signature, signature, 103),
    (Timestamp, timestamp, 107),
    (Address, address, 110),
);

impl From<Type> for super::Type {
    fn from(value: Type) -> Self {
        Self::Comparable(value)
    }
}

impl TryFrom<super::Type> for Type {
    type Error = Error;

    fn try_from(value: super::Type) -> Result<Self> {
        if let super::Type::Comparable(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonType)
    }
}

impl From<Type> for Michelson {
    fn from(value: Type) -> Self {
        Self::Type(value.into())
    }
}

impl TryFrom<Michelson> for Type {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        let value: super::Type = value.try_into()?;
        value.try_into()
    }
}

impl From<Primitive> for crate::michelson::Primitive {
    fn from(value: Primitive) -> Self {
        Self::ComparableType(value)
    }
}
