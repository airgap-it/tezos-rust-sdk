use std::string::FromUtf8Error;

use derive_more::{Display, Error as DError, From};

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Core { source: tezos_core::Error },
    Michelson { source: tezos_michelson::Error },
    InvalidOperationContentTag,
    InvalidBytes,
    InvalidStringConversion { source: FromUtf8Error },
}

pub type Result<T> = std::result::Result<T, Error>;
