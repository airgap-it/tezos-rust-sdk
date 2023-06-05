#![cfg_attr(not(feature = "std"), no_std)]

//! The `tezos-michelson` crate provides various features to make working with `Micheline` expressions easier. It can help developers
//! to deserialize and serialize expressions from/to JSON, pack and unpack it using an optional schema or convert it to
//! a better typed `Michelson` structure.
//!
//! ## JSON Serialization
//!
//! Micheline types can be serialized/deserialized to/from JSON using the `serde` serialization library (requires the `serde` feature):
//!
//! ```rust
//! use tezos_michelson::micheline::Micheline;
//! use serde_json::{json, Value};
//!
//! let json_value = json!({ "int": "10" });
//! # #[cfg(feature = "serde")]
//! let micheline: Micheline = serde_json::from_value(json_value).expect("Micheline structure can be constructed from valid json representation");
//! # #[cfg(feature = "serde")]
//! let json: String = serde_json::to_string(&micheline).expect("Micheline structure can be serialized to a JSON string");
//! ```
//!
//! ## Create an Expression
//!
//! Sometimes it may be useful to create a Micheline expression directly in the code, for example to define a packing schema
//! or provide contract call parameters.
//!
//! The examples below present how to create [Micheline](crate::micheline::Micheline) expressions. Each example features a reference JSON to help you
//! understand what the final expression is going to look like.
//!
//! ### Literals
//!
//! #### Int
//!
//! JSON
//! ```json
//! {
//!   "int": "10"
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::micheline::{int, Micheline};
//!
//! let int: Micheline = int(10);
//! ```
//!
//! #### String
//!
//! JSON
//! ```json
//! {
//!   "string": "value"
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::micheline::{try_string, Micheline};
//!
//! let string: Micheline = try_string("value").expect("valid conversion to Micheline");
//! ```
//!
//! #### Bytes
//!
//! JSON
//! ```json
//! {
//!   "bytes": "0a"
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::micheline::{bytes, try_bytes, Micheline};
//!
//! let bytes_value_1: Micheline = try_bytes("0a").expect("valid conversion to Micheline");
//! let bytes_value_2: Micheline = bytes(vec![0u8, 0u8, 1u8]);
//! ```
//!
//! ### Primitive Application
//!
//! #### Data
//!
//! JSON
//! ```json
//! {
//!   "prim": "Pair",
//!   "args": [
//!     {
//!       "prim": "Some",
//!       "args": [
//!         {
//!           "int": "10"
//!         }
//!       ]
//!     },
//!     {
//!       "prim": "Left",
//!       "args": [
//!         {
//!           "string": "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e"
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::{micheline::{Micheline, primitive_application, int, try_string}, michelson::DataPrimitive};
//!
//! let pair: Micheline = primitive_application(DataPrimitive::Pair)
//!                         .with_args(vec![
//!                             primitive_application(DataPrimitive::Some)
//!                                 .with_args(vec![int(10)]).into(),
//!                             primitive_application(DataPrimitive::Left)
//!                                 .with_args(vec![try_string("tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e").unwrap()]).into(),
//!                         ]).into();
//! ```
//!
//! #### Instruction
//!
//! JSON
//! ```json
//! {
//!   "prim": "MAP",
//!   "args": [
//!     [
//!       {
//!         "prim": "DIG",
//!         "args": [
//!           {
//!             "int": "2"
//!           }
//!         ]
//!       },
//!       {
//!         "prim": "DUP"
//!       },
//!       {
//!         "prim": "DUG",
//!         "args": [
//!           {
//!             "int": "3"
//!           }
//!         ]
//!       },
//!       {
//!         "prim": "SWAP"
//!       }
//!     ]
//!   ]
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::{micheline::{Micheline, primitive_application, int, sequence}, michelson::InstructionPrimitive};
//!
//! let map: Micheline = primitive_application(InstructionPrimitive::Map)
//!                         .with_args(vec![
//!                             sequence(vec![
//!                                 primitive_application(InstructionPrimitive::Dig)
//!                                     .with_args(vec![int(2)]).into(),
//!                                 primitive_application(InstructionPrimitive::Dup).into(),
//!                                 primitive_application(InstructionPrimitive::Dug)
//!                                     .with_args(vec![int(3)]).into(),
//!                                 primitive_application(InstructionPrimitive::Swap).into(),
//!                             ])
//!                         ]).into();
//! ```
//!
//! #### Types
//!
//! JSON
//! ```json
//! {
//!   "prim": "pair",
//!   "args": [
//!     {
//!       "prim": "option",
//!       "args": [
//!         {
//!           "prim": "nat",
//!           "annots": [
//!             "%nat"
//!           ]
//!         }
//!       ]
//!     },
//!     {
//!       "prim": "or",
//!       "args": [
//!         {
//!           "prim": "address",
//!           "annots": [
//!             "%address"
//!           ]
//!         },
//!         {
//!           "prim": "bytes",
//!           "annots": [
//!             "%bytes"
//!           ]
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::{micheline::{Micheline, primitive_application}, michelson::{TypePrimitive, ComparableTypePrimitive}};
//!
//! let pair: Micheline = primitive_application(ComparableTypePrimitive::Pair)
//!                         .with_args(vec![
//!                             primitive_application(ComparableTypePrimitive::Option)
//!                                 .with_args(vec![
//!                                     primitive_application(ComparableTypePrimitive::Nat)
//!                                         .with_annots(vec!["%nat".into()]).into(),
//!                                 ]).into(),
//!                             primitive_application(ComparableTypePrimitive::Or)
//!                                 .with_args(vec![
//!                                     primitive_application(ComparableTypePrimitive::Address)
//!                                         .with_annots(vec!["%address".into()]).into(),
//!                                     primitive_application(ComparableTypePrimitive::Bytes)
//!                                         .with_annots(vec!["%bytes".into()]).into(),
//!                                 ]).into(),
//!                         ]).into();
//! ```
//!
//! ### Sequences
//!
//! JSON
//! ```json
//! [
//!   {
//!     "prim": "DUP"
//!   },
//!   {
//!     "prim": "CDR"
//!   },
//!   {
//!     "prim": "SWAP"
//!   }
//! ]
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::{micheline::{Micheline, sequence, primitive_application}, michelson::InstructionPrimitive};
//!
//! let seq: Micheline = sequence(vec![
//!     primitive_application(InstructionPrimitive::Dup).into(),
//!     primitive_application(InstructionPrimitive::Cdr).into(),
//!     primitive_application(InstructionPrimitive::Swap).into(),
//! ]);
//! ```
//!
//! ## Pack and Unpack
//!
//! To serialize a Micheline expression to its optimized binary representation use [Micheline::pack](crate::micheline::Micheline::pack),
//! to deserialize a Micheline expression from the optimized binary representation use `init(fromPacked:)`:
//!
//! ```rust
//! use tezos_michelson::{micheline::{Micheline, try_string, primitive_application}, michelson::ComparableTypePrimitive};
//!
//! let micheline: Micheline = try_string("tz1ZBuF2dQ7E1b32bK3g1Qsah4pvWqpM4b4A").expect("valid conversion to Micheline");
//! let schema: Micheline = primitive_application(ComparableTypePrimitive::Address).into();
//!
//! let packed_bytes = micheline.pack(Some(&schema)).expect("valid conversion to packed bytes"); // = [5, 10, 0, 0, 0, 22, 0, 0, 148, 160, 186, 39, 22, 158, 216, 217, 124, 31, 71, 109, 230, 21, 108, 36, 130, 219, 251, 61]
//! let unpacked_bytes = Micheline::from_bytes(&packed_bytes).expect("valid conversion to Micheline"); // = { "string": "tz1ZBuF2dQ7E1b32bK3g1Qsah4pvWqpM4b4A" }
//! ```
//!
//! # `Michelson` (type)
//!
//! The [Michelson](crate::michelson::Michelson) type is the representation of the Smart Contract language. It provides a much
//! more strictly typed interface than [Micheline](crate::micheline::Micheline) and it can be converted to a Micheline expression
//! or created out of it.
//!
//! ## Create Expression
//!
//! [Michelson](crate::michelson::Michelson) splits into [Data](crate::michelson::data::Data), [Instruction](crate::michelson::data::instructions::Instruction), [Type](crate::michelson::types::Type) and [ComparableType](crate::michelson::types::ComparableType)
//! types, defined based on [the grammar specification](https://tezos.gitlab.io/active/michelson.html#full-grammar):
//!
//! - [Michelson](crate::michelson::Michelson)
//!   -  [Data](crate::michelson::data::Data)
//!     - ...
//!     - [Instruction](crate::michelson::data::instructions::Instruction)
//!       - ...
//!   - [Type](crate::michelson::types::Type)
//!     - ...
//!     - [ComparableType](crate::michelson::types::ComparableType)
//!       - ...
//!
//! The examples below present how to create Michelson expressions. Each example features a reference JSON to help you
//! understand what the final expression (in Micheline) is going to look like.
//!
//! ### Data
//!
//! JSON
//! ```json
//! {
//!   "prim": "Pair",
//!   "args": [
//!     {
//!       "prim": "Some",
//!       "args": [
//!         {
//!           "int": "10"
//!         }
//!       ]
//!     },
//!     {
//!       "prim": "Left",
//!       "args": [
//!         {
//!           "string": "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e"
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::michelson::{Michelson, data::{pair, some, left, int, try_string}};
//!
//! let michelson: Michelson = pair(vec![
//!     some(int(10)),
//!     left(try_string("tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e").unwrap()),
//! ]);
//!
//! ```
//!
//! ### Instruction
//!
//! JSON
//! ```json
//! {
//!   "prim": "MAP",
//!   "args": [
//!     [
//!       {
//!         "prim": "DIG",
//!         "args": [
//!           {
//!             "int": "2"
//!           }
//!         ]
//!       },
//!       {
//!         "prim": "DUP"
//!       },
//!       {
//!         "prim": "DUG",
//!         "args": [
//!           {
//!             "int": "3"
//!           }
//!         ]
//!       },
//!       {
//!         "prim": "SWAP"
//!       }
//!     ]
//!   ]
//! }
//! ```
//!
//! Expression
//! ```rust
//! use tezos_michelson::michelson::{Michelson, data::{instructions::{map, dig, dup, dug, swap}, int}};
//!
//! let michelson: Michelson = map(
//!     vec![
//!         dig(2u8.into()),
//!         dup(None),
//!         dug(3u8.into()),
//!         swap(),
//!     ].into()
//! );
//! ```
//!
//! ### Type
//!
//! JSON
//! ```json
//! {
//!   "prim": "pair",
//!   "args": [
//!     {
//!       "prim": "option",
//!       "args": [
//!         {
//!           "prim": "nat",
//!           "annots": [
//!             "%nat"
//!           ]
//!         }
//!       ]
//!     },
//!     {
//!       "prim": "or",
//!       "args": [
//!         {
//!           "prim": "address",
//!           "annots": [
//!             "%address"
//!           ]
//!         },
//!         {
//!           "prim": "chest_key",
//!           "annots": [
//!             "%chest_key"
//!           ]
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! ```rust
//! use tezos_michelson::michelson::{Michelson, types::{pair, option, Nat, nat, or, Address, address, ChestKey, chest_key}};
//!
//! let michelson: Michelson = pair(vec![
//!     option(nat::<Nat>().with_field_annotation("nat".into())),
//!     or(
//!         address::<Address>().with_field_annotation("address".into()),
//!         chest_key::<ChestKey>().with_field_annotation("chest_key".into()),
//!     ),
//! ]);
//! ```
//!
//! ### ComparableType
//!
//! JSON
//! ```json
//! {
//!   "prim": "pair",
//!   "args": [
//!     {
//!       "prim": "option",
//!       "args": [
//!         {
//!           "prim": "nat",
//!           "annots": [
//!             "%nat"
//!           ]
//!         }
//!       ]
//!     },
//!     {
//!       "prim": "or",
//!       "args": [
//!         {
//!           "prim": "address",
//!           "annots": [
//!             "%address"
//!           ]
//!         },
//!         {
//!           "prim": "bytes",
//!           "annots": [
//!             "%bytes"
//!           ]
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! ```rust
//! use tezos_michelson::michelson::{Michelson, types::{comparable_pair, comparable_or, comparable_option, Nat, nat, Address, address, Bytes, bytes}};
//!
//! let michelson: Michelson = comparable_pair(vec![
//!     comparable_option(nat::<Nat>().with_field_annotation("nat".into())),
//!     comparable_or(
//!         address::<Address>().with_field_annotation("address".into()),
//!         bytes::<Bytes>().with_field_annotation("bytes".into()),
//!     ),
//! ]);
//! ```
//!
//! ## Micheline Conversion
//!
//! To convert values between the [Michelson](crate::michelson::Michelson) and [Micheline](crate::micheline::Micheline) types use the`From` trait conformance:
//!
//! ```rust
//! use tezos_michelson::{micheline::Micheline, michelson::{Michelson, types::{pair, option, or, nat, unit, map, bytes, address}}};
//!
//! let pair: Michelson = pair(vec![
//!     option(nat()),
//!     or(
//!         unit(),
//!         map(bytes(), address()),
//!     ),
//! ]);
//!
//! let micheline: Micheline = pair.into();
//! let michelson: Michelson = micheline.try_into().expect("valid conversion to Michelson");
//! ```
extern crate alloc;

mod common;
mod error;
mod internal;
pub mod micheline;
pub mod michelson;

pub use error::{Error, Result};
pub use internal::packer::MichelinePacker;

#[cfg(test)]
mod test {
    use crate::{
        micheline::{self, Micheline},
        michelson::{self, Michelson},
        Result,
    };

    #[test]
    fn test_michelson_to_micheline() -> Result<()> {
        for (michelson, expected) in michelson_micheline_values() {
            let micheline: Micheline = michelson.into();
            assert_eq!(expected, micheline);
        }
        Ok(())
    }

    #[test]
    fn test_micheline_to_michelson() -> Result<()> {
        for (expected, micheline) in michelson_micheline_values() {
            let michelson: Michelson = micheline.try_into()?;
            assert_eq!(expected, michelson);
        }
        Ok(())
    }

    fn michelson_micheline_values() -> Vec<(Michelson, Micheline)> {
        vec![
            (michelson::data::int(1), micheline::int(1)),
            (
                michelson::data::try_string("string").unwrap(),
                micheline::try_string("string").unwrap(),
            ),
            (
                michelson::data::try_bytes("0x00").unwrap(),
                micheline::try_bytes("0x00").unwrap(),
            ),
            (
                michelson::data::unit(),
                micheline::primitive_application("Unit").into(),
            ),
            (
                michelson::data::r#true(),
                micheline::primitive_application("True").into(),
            ),
            (
                michelson::data::r#false(),
                micheline::primitive_application("False").into(),
            ),
            (
                michelson::data::pair(vec![true.into(), false.into()]),
                micheline::primitive_application("Pair")
                    .with_args(vec![
                        micheline::primitive_application("True").into(),
                        micheline::primitive_application("False").into(),
                    ])
                    .into(),
            ),
            (
                michelson::data::left(().into()),
                micheline::primitive_application("Left")
                    .with_args(vec![micheline::primitive_application("Unit").into()])
                    .into(),
            ),
            (
                michelson::data::right(().into()),
                micheline::primitive_application("Right")
                    .with_args(vec![micheline::primitive_application("Unit").into()])
                    .into(),
            ),
            (
                michelson::data::some(().into()),
                micheline::primitive_application("Some")
                    .with_args(vec![micheline::primitive_application("Unit").into()])
                    .into(),
            ),
            (
                michelson::data::none(),
                micheline::primitive_application("None").into(),
            ),
            (
                vec![().into()].into(),
                vec![micheline::primitive_application("Unit").into()].into(),
            ),
            (
                michelson::data::sequence(vec![michelson::data::instructions::unit(), ().into()]),
                vec![
                    micheline::primitive_application("UNIT").into(),
                    micheline::primitive_application("Unit").into(),
                ]
                .into(),
            ),
            (
                michelson::types::unit(),
                micheline::primitive_application("unit").into(),
            ),
        ]
    }
}
