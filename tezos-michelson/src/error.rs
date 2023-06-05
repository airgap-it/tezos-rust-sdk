use core::result;

use alloc::string::String;
use derive_more::{Display, From};

#[cfg(feature = "std")]
use derive_more::Error as DError;

/// Errors returned by this crate.
#[derive(Display, Debug, From)]
#[cfg_attr(feature = "std", derive(DError))]
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
