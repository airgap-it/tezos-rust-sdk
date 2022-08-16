use std::result;

use derive_more::{Display, Error as DError, From};

/// Errors returned by this crate.
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
    #[from(ignore)]
    InvalidMicheline {
        description: String,
    },
    InvalidPrimitiveApplication,
    InvalidMichelineLiteral,
    MichelineValueSchemaMismatch,
}

pub type Result<T> = result::Result<T, Error>;
