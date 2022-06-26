use derive_more::{Display, Error as DError, From};
use std::result;

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Internal {
        description: String,
    },
    InvalidBase58EncodedData,
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
}

pub type Result<T> = result::Result<T, Error>;
