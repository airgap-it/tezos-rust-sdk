use {crate::models::error::RPCError, derive_more::From};

#[derive(Debug, From)]
pub enum Error {
    Core {
        source: tezos_core::Error,
    },
    HTTPError {
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
    RPCErrorPlain(String),
    RPCErrors(Vec<RPCError>),
}
