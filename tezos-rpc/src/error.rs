#[cfg(feature = "std")]
use derive_more::Error as DError;
use {
    crate::models::error::RpcErrors,
    derive_more::{Display, From},
};

#[derive(Debug, From, Display)]
#[cfg_attr(feature = "std", derive(DError))]
pub enum Error {
    Core {
        source: tezos_core::Error,
    },
    Operation {
        source: tezos_operation::Error,
    },
    #[cfg(feature = "http")]
    HttpError {
        source: reqwest::Error,
    },
    ParsingError {
        source: serde_json::Error,
    },
    ParseIntError {
        source: std::num::ParseIntError,
    },
    ParseBigIntError {
        source: num_bigint::ParseBigIntError,
    },
    RpcErrorPlain {
        description: String,
    },
    RpcErrors(#[cfg_attr(feature = "std", error(not(source)))] RpcErrors),
    InvalidConversion,
    OperationNotSupported,
}

pub type Result<T> = std::result::Result<T, Error>;
