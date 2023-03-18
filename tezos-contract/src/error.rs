use std::result;

#[cfg(feature = "std")]
use derive_more::Error as DError;
use derive_more::{Display, From};

#[derive(Display, Debug, From)]
#[cfg_attr(feature = "std", derive(DError))]
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
