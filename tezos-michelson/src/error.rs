use std::result;

use derive_more::{Display, Error as DError, From};

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Internal {
        description: String,
    },
    Core {
        source: tezos_core::Error,
    },
    InvalidAnnotationString,
    InvalidAnnotation,
    InvalidIntString,
    InvalidNatString,
    InvalidStringValue,
    InvalidHexString,
    InvalidVecValue,
    InvalidMichelson,
    InvalidMichelsonData,
    InvalidMichelsonInstruction,
    InvalidMichelsonType,
    InvalidMichelsonComparableType,
    UnknownMichelsonPrimName,
    UnknownMichelsonPrimTag,
    InvalidBytes,
    InvalidMicheline,
    InvalidPrimitiveApplication,
    InvalidMichelineLiteral,
    MichelineValueSchemaMismatch,
    #[cfg(feature = "serde")]
    JSON {
        source: serde_json::Error,
    },
}

pub type Result<T> = result::Result<T, Error>;
