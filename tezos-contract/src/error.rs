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
    Michelson {
        source: tezos_michelson::Error,
    },
    Rpc {
        source: tezos_rpc::Error,
    },
    InvalidContractScript,
    EntrypointNotFound,
    #[from(ignore)]
    IncompatibleValue {
        description: String,
    },
}

pub type Result<T> = result::Result<T, Error>;
