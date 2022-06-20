pub mod comparables;
mod macros;

use macros::{make_type, make_types};

use super::Michelson;
use crate::{Error, Result};

make_types!(
    Comparable(crate::michelson::types::comparables::Type),
    [
        pub fn prim_values() -> Vec<&'static crate::michelson::Prim> {
            [PRIMS, crate::michelson::types::comparables::Type::prim_values()].concat()
        }

        pub fn is_valid_prim_name(name: &str) -> bool {
            Self::prim_values()
                .iter()
                .find(|prim| prim.name() == name)
                .is_some()
        }

        fn fallback(value: PrimitiveApplication) -> Result<Self> {
            Ok(Self::Comparable(value.try_into()?))
        }
    ],
    conversion_fallback: fallback,
    (Parameter, parameter, 0, boxed: (r#type: Type)),
    (Storage, storage, 1, boxed: (r#type: Type)),
    (
        Code,
        code,
        2,
        boxed: (code: crate::michelson::data::instructions::Instruction)
    ),
    (Option, option, 99, boxed: (r#type: Type)),
    (List, list, 95, boxed: (r#type: Type)),
    (
        Set,
        set,
        102,
        (r#type: crate::michelson::types::comparables::Type)
    ),
    (Operation, operation, 109),
    (Contract, contract, 90, boxed: (r#type: Type)),
    (Ticket, ticket, 135, boxed: (r#type: Type)),
    (Pair, pair, 101, vec: (types: Type)),
    (Or, or, 100, boxed: (lhs: Type), boxed: (rhs: Type)),
    (
        Lambda,
        lambda,
        94,
        boxed: (parameter_type: Type),
        boxed: (return_type: Type)
    ),
    (Map, map, 96, boxed: (key_type: Type), boxed: (value_type: Type)),
    (
        BigMap,
        big_map,
        97,
        boxed: (key_type: Type),
        boxed: (value_type: Type)
    ),
    (Bls12_381G1, bls12_381_g1, 128),
    (Bls12_381G2, bls12_381_g2, 129),
    (Bls12_381Fr, bls12_381_fr, 130),
    (
        SaplingTransaction,
        sapling_transaction,
        132,
        (memo_size: crate::michelson::data::Nat)
    ),
    (
        SaplingState,
        sapling_state,
        131,
        (memo_size: crate::michelson::data::Nat)
    ),
    (Chest, chest, 141),
    (ChestKey, chest_key, 142),
);

impl From<Type> for Michelson {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}

impl TryFrom<Michelson> for Type {
    type Error = Error;

    fn try_from(value: Michelson) -> Result<Self> {
        if let Michelson::Type(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelson)
    }
}
