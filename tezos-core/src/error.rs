use derive_more::{Display, From};
use std::{result, string::FromUtf8Error};

#[cfg(feature = "std")]
use derive_more::Error as DError;

/// Errors returned by this crate.
#[derive(Display, Debug, From)]
#[cfg_attr(feature = "std", derive(DError))]
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
    TryFromBigInt {
        source: num_bigint::TryFromBigIntError<num_bigint::BigInt>,
    },
    TryFromBigUInt {
        source: num_bigint::TryFromBigIntError<num_bigint::BigUint>,
    },
    Blake2InvalidOutputSize {
        source: blake2::digest::InvalidOutputSize,
    },
    Blake2InvalidBufferSize {
        source: blake2::digest::InvalidBufferSize,
    },
    InvalidSecretKeyBytes,
    InvalidPublicKeyBytes,
    InvalidSignatureBytes,
    InvalidIntegerConversion,
    InvalidNaturalConversion,
    InvalidAddress,
    InvalidContractAddress,
    InvalidHexString,
    CryptoProviderNotSet,
    #[cfg(feature = "secp256_k1")]
    Secp256K1Signing {
        source: k256::ecdsa::signature::Error,
    },
    #[from(ignore)]
    #[cfg(feature = "p256")]
    P256Signing {
        source: p256::ecdsa::signature::Error,
    },
}

pub type Result<T> = result::Result<T, Error>;
