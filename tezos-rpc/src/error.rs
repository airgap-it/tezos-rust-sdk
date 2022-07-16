use {crate::models::error::RpcError, derive_more::From};

#[derive(Debug, From)]
pub enum Error {
    Core {
        source: tezos_core::Error,
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
    RpcErrorPlain(String),
    RpcErrors(Vec<RpcError>),
}
