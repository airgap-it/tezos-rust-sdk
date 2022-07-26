use crate::{michelson::Michelson, Error, Result};

use super::macros::{make_type, make_types};

make_types!(
    [
        fn fallback(_value: PrimitiveApplication) -> Result<Self> {
            Err(Error::InvalidPrimitiveApplication)
        }
    ],
    conversion_fallback: fallback,
    (Unit, unit, 108, super_enum: crate::michelson::types::Type, Comparable),
    (Never, never, 120, super_enum: crate::michelson::types::Type, Comparable),
    (Bool, bool, 89, super_enum: crate::michelson::types::Type, Comparable),
    (Int, int, 91, super_enum: crate::michelson::types::Type, Comparable),
    (Nat, nat, 98, super_enum: crate::michelson::types::Type, Comparable),
    (String, string, 104, super_enum: crate::michelson::types::Type, Comparable),
    (ChainId, chain_id, 116, super_enum: crate::michelson::types::Type, Comparable),
    (Bytes, bytes, 105, super_enum: crate::michelson::types::Type, Comparable),
    (Mutez, mutez, 106, super_enum: crate::michelson::types::Type, Comparable),
    (KeyHash, key_hash, 93, super_enum: crate::michelson::types::Type, Comparable),
    (Key, key, 92, super_enum: crate::michelson::types::Type, Comparable),
    (Signature, signature, 103, super_enum: crate::michelson::types::Type, Comparable),
    (Timestamp, timestamp, 107, super_enum: crate::michelson::types::Type, Comparable),
    (Address, address, 110, super_enum: crate::michelson::types::Type, Comparable),
    (Option, option, 99, super_enum: crate::michelson::types::Type, Comparable, boxed: (r#type: Type)),
    (Or, or, 100, super_enum: crate::michelson::types::Type, Comparable, boxed: (lhs: Type), boxed: (rhs: Type)),
    (Pair, pair, 101, super_enum: crate::michelson::types::Type, Comparable, vec: (types: Type)),
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
