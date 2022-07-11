use derive_more::{Display, Error as DError, From};

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Core { source: tezos_core::Error },
}

pub type Result<T> = std::result::Result<T, Error>;
