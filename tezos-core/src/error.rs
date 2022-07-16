use derive_more::{Display, Error as DError, From};
use std::{result, string::FromUtf8Error};

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Internal {
        description: String,
    },
    #[from(ignore)]
    InvalidBase58EncodedData {
        description: String,
    },
    InvalidBytes,
    Base58Decoding {
        source: bs58::decode::Error,
    },
    InvalidIntegerString,
    InvalidUnsignedIntegerString,
    InvalidTezString,
    BigIntParse {
        source: num_bigint::ParseBigIntError,
    },
    IntParse {
        source: std::num::ParseIntError,
    },
    InvalidStringConversion {
        source: FromUtf8Error,
    },
    InvalidConversion,
    InvalidEncodedValue,
    InvalidNaturalBytes,
    InvalidIntegerBytes,
    TryFromInt {
        source: std::num::TryFromIntError,
    },
    InvalidSecretKeyBytes,
    InvalidPublicKeyBytes,
    InvalidSignatureBytes,
    InvalidIntegerConversion,
    InvalidNaturalConversion,
    InvalidAddress,
    InvalidContractAddress,
    InvalidHexString,
}

pub type Result<T> = result::Result<T, Error>;
