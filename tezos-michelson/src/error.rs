use std::{result, string::FromUtf8Error};

use derive_more::{Display, Error as DError, From};

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Internal { description: String },
    Core { source: tezos_core::Error },
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
    InvalidStringConversion { source: FromUtf8Error },
    InvalidMicheline,
    InvalidPrimitiveApplication,
    InvalidMichelineLiteral,
    JSON { source: serde_json::Error },
}

pub type Result<T> = result::Result<T, Error>;
