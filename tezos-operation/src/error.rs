use std::string::FromUtf8Error;

#[cfg(feature = "std")]
use derive_more::Error as DError;
use derive_more::{Display, From};

#[derive(Display, Debug, From)]
#[cfg_attr(feature = "std", derive(DError))]
pub enum Error {
    Core { source: tezos_core::Error },
    Michelson { source: tezos_michelson::Error },
    InvalidOperationContentTag,
    InvalidBytes,
    InvalidStringConversion { source: FromUtf8Error },
}

pub type Result<T> = std::result::Result<T, Error>;
