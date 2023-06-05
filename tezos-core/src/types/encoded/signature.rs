#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    internal::{
        coder::{ConsumingDecoder, EncodedBytesCoder},
        consumable_list::ConsumableList,
    },
    types::encoded::{
        ed25519_signature::Ed25519Signature, generic_signature::GenericSignature,
        p256_signature::P256Signature, secp256_k1_signature::Secp256K1Signature, Encoded,
        MetaEncoded,
    },
    Error, Result,
};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

/// Group of base58 encoded signatures.
///
/// See:
/// - [GenericSignature]
/// - [Ed25519Signature]
/// - [Secp256K1Signature]
/// - [P256Signature]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String", untagged)
)]
pub enum Signature {
    Generic(GenericSignature),
    Ed25519(Ed25519Signature),
    Secp256K1(Secp256K1Signature),
    P256(P256Signature),
}

impl Signature {
    pub fn to_generic_signature(self) -> Result<GenericSignature> {
        match self {
            Signature::Generic(value) => Ok(value),
            Signature::Ed25519(value) => (&value.to_bytes()?).try_into(),
            Signature::Secp256K1(value) => (&value.to_bytes()?).try_into(),
            Signature::P256(value) => (&value.to_bytes()?).try_into(),
        }
    }
}

impl Encoded for Signature {
    type Coder = EncodedBytesCoder;

    fn value(&self) -> &str {
        match self {
            Self::Generic(value) => value.value(),
            Self::Ed25519(value) => value.value(),
            Self::Secp256K1(value) => value.value(),
            Self::P256(value) => value.value(),
        }
    }

    fn meta(&self) -> &'static MetaEncoded {
        match self {
            Self::Generic(value) => value.meta(),
            Self::Ed25519(value) => value.meta(),
            Self::Secp256K1(value) => value.meta(),
            Self::P256(value) => value.meta(),
        }
    }

    fn new(base58: String) -> crate::Result<Self> {
        if GenericSignature::is_valid_base58(&base58) {
            return Ok(Self::Generic(GenericSignature::new(base58)?));
        }
        if Ed25519Signature::is_valid_base58(&base58) {
            return Ok(Self::Ed25519(Ed25519Signature::new(base58)?));
        }
        if Secp256K1Signature::is_valid_base58(&base58) {
            return Ok(Self::Secp256K1(Secp256K1Signature::new(base58)?));
        }
        if P256Signature::is_valid_base58(&base58) {
            return Ok(Self::P256(P256Signature::new(base58)?));
        }
        Err(Error::InvalidBase58EncodedData {
            description: base58,
        })
    }

    fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        match self {
            Self::Generic(value) => value.to_bytes(),
            Self::Ed25519(value) => value.to_bytes(),
            Self::Secp256K1(value) => value.to_bytes(),
            Self::P256(value) => value.to_bytes(),
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if GenericSignature::is_valid_bytes(bytes) {
            return Ok(Self::Generic(GenericSignature::from_bytes(bytes)?));
        }
        if Ed25519Signature::is_valid_bytes(bytes) {
            return Ok(Self::Ed25519(Ed25519Signature::from_bytes(bytes)?));
        }
        if Secp256K1Signature::is_valid_bytes(bytes) {
            return Ok(Self::Secp256K1(Secp256K1Signature::from_bytes(bytes)?));
        }
        if P256Signature::is_valid_bytes(bytes) {
            return Ok(Self::P256(P256Signature::from_bytes(bytes)?));
        }

        Ok(Self::Generic(GenericSignature::from_bytes(bytes)?))
    }

    fn from_consumable_bytes<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Self>
    where
        Self::Coder: ConsumingDecoder<Self, u8, Error>,
    {
        if GenericSignature::is_valid_prefixed_consumable_bytes(bytes.inner_value()) {
            return Ok(Self::Generic(GenericSignature::from_consumable_bytes(
                bytes,
            )?));
        }
        if Ed25519Signature::is_valid_prefixed_consumable_bytes(bytes.inner_value()) {
            return Ok(Self::Ed25519(Ed25519Signature::from_consumable_bytes(
                bytes,
            )?));
        }
        if Secp256K1Signature::is_valid_prefixed_consumable_bytes(bytes.inner_value()) {
            return Ok(Self::Secp256K1(Secp256K1Signature::from_consumable_bytes(
                bytes,
            )?));
        }
        if P256Signature::is_valid_prefixed_consumable_bytes(bytes.inner_value()) {
            return Ok(Self::P256(P256Signature::from_consumable_bytes(bytes)?));
        }

        Ok(Self::Generic(GenericSignature::from_consumable_bytes(
            bytes,
        )?))
    }
}

impl From<GenericSignature> for Signature {
    fn from(value: GenericSignature) -> Self {
        Self::Generic(value)
    }
}

impl From<Ed25519Signature> for Signature {
    fn from(value: Ed25519Signature) -> Self {
        Self::Ed25519(value)
    }
}

impl From<Secp256K1Signature> for Signature {
    fn from(value: Secp256K1Signature) -> Self {
        Self::Secp256K1(value)
    }
}

impl From<P256Signature> for Signature {
    fn from(value: P256Signature) -> Self {
        Self::P256(value)
    }
}

impl TryFrom<&Vec<u8>> for Signature {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        Self::from_bytes(value)
    }
}

impl TryFrom<String> for Signature {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Signature::new(value)
    }
}

impl TryFrom<&str> for Signature {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Signature::new(value.to_string())
    }
}

impl TryFrom<&Signature> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Signature) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_generic() -> Result<()> {
        let signature: Signature = "edsigtczTq2EC9VQNRRT53gzcs25DJFg1iZeTzQxY7jBtjradZb8qqZaqzAYSbVWvg1abvqFpQCV8TgqotDwckJiTJ9fJ2eYESb".try_into()?;
        let generic = signature.to_generic_signature()?;
        assert_eq!(generic.value(), "sigTAzhy1HsZDLNETmuf9RuinhXRb5jvmscjCoPPBujWZgFmCFLffku7JXYtu8aYQFVHnCUghmd4t39RuR6ANV76bCCYTR9u");
        Ok(())
    }
}
