#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    contract_hash::ContractHash, ed25519_public_key_hash::Ed25519PublicKeyHash,
    p256_public_key_hash::P256PublicKeyHash, secp256_k1_public_key_hash::Secp256K1PublicKeyHash,
    Encoded, MetaEncoded, TraitMetaEncoded,
};
use crate::{
    internal::coder::{AddressBytesCoder, ContractAddressBytesCoder, ImplicitAddressBytesCoder},
    Error, Result,
};
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

/// Group of base58 encoded Tezos account addresses, either implicit or originated.
///
/// See:
/// - [ImplicitAddress]
/// - [ContractAddress]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String", untagged)
)]
pub enum Address {
    Implicit(ImplicitAddress),
    Originated(ContractAddress),
}

impl Encoded for Address {
    type Coder = AddressBytesCoder;

    fn value(&self) -> &str {
        match self {
            Self::Implicit(address) => address.value(),
            Self::Originated(address) => address.value(),
        }
    }

    fn meta(&self) -> &'static MetaEncoded {
        match self {
            Self::Implicit(address) => address.meta(),
            Self::Originated(address) => address.meta(),
        }
    }

    fn new(value: String) -> Result<Self> {
        if ImplicitAddress::is_valid_base58(&value) {
            return Ok(Self::Implicit(ImplicitAddress::new(value)?));
        }
        if ContractAddress::is_valid_base58(&value) {
            return Ok(Self::Originated(ContractAddress::new(value)?));
        }
        Err(Error::InvalidBase58EncodedData { description: value })
    }
}

impl From<Address> for String {
    fn from(value: Address) -> Self {
        match value {
            Address::Implicit(value) => value.into(),
            Address::Originated(value) => value.into(),
        }
    }
}

impl TryFrom<&Vec<u8>> for Address {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        Self::from_bytes(value)
    }
}

impl TryFrom<String> for Address {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Address::new(value)
    }
}

impl TryFrom<&str> for Address {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Address::new(value.to_string())
    }
}

impl TryFrom<&Address> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Address) -> Result<Self> {
        value.to_bytes()
    }
}

impl From<ImplicitAddress> for Address {
    fn from(value: ImplicitAddress) -> Self {
        Self::Implicit(value)
    }
}

impl From<ContractAddress> for Address {
    fn from(value: ContractAddress) -> Self {
        Self::Originated(value)
    }
}

impl From<&ContractHash> for Address {
    fn from(value: &ContractHash) -> Self {
        let contract_address = ContractAddress::from_components(value, None);
        contract_address.into()
    }
}

impl TryFrom<Address> for ContractAddress {
    type Error = Error;

    fn try_from(value: Address) -> Result<Self> {
        if let Address::Originated(value) = value {
            return Ok(value);
        }

        Err(Error::InvalidAddress)
    }
}

impl TryFrom<Address> for ContractHash {
    type Error = Error;

    fn try_from(value: Address) -> Result<Self> {
        let contract_address: ContractAddress = value.try_into()?;
        contract_address.contract_hash().try_into()
    }
}

/// Group of base58 encoded implicit Tezos account addresses.
///
/// See:
/// - [Ed25519PublicKeyHash]
/// - [Secp256K1PublicKeyHash]
/// - [P256PublicKeyHash]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String", untagged)
)]
pub enum ImplicitAddress {
    TZ1(Ed25519PublicKeyHash),
    TZ2(Secp256K1PublicKeyHash),
    TZ3(P256PublicKeyHash),
}

impl ImplicitAddress {
    pub fn is_valid_base58(value: &str) -> bool {
        Ed25519PublicKeyHash::is_valid_base58(value)
            || Secp256K1PublicKeyHash::is_valid_base58(value)
            || P256PublicKeyHash::is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        Ed25519PublicKeyHash::is_valid_prefixed_bytes(value)
            || Secp256K1PublicKeyHash::is_valid_prefixed_bytes(value)
            || P256PublicKeyHash::is_valid_prefixed_bytes(value)
    }
}

impl Encoded for ImplicitAddress {
    type Coder = ImplicitAddressBytesCoder;

    fn value(&self) -> &str {
        match self {
            Self::TZ1(address) => address.value(),
            Self::TZ2(address) => address.value(),
            Self::TZ3(address) => address.value(),
        }
    }

    fn meta(&self) -> &'static MetaEncoded {
        match self {
            Self::TZ1(address) => address.meta(),
            Self::TZ2(address) => address.meta(),
            Self::TZ3(address) => address.meta(),
        }
    }

    fn new(value: String) -> Result<Self> {
        if Ed25519PublicKeyHash::is_valid_base58(&value) {
            return Ok(Self::TZ1(Ed25519PublicKeyHash::new(value)?));
        }
        if Secp256K1PublicKeyHash::is_valid_base58(&value) {
            return Ok(Self::TZ2(Secp256K1PublicKeyHash::new(value)?));
        }
        if P256PublicKeyHash::is_valid_base58(&value) {
            return Ok(Self::TZ3(P256PublicKeyHash::new(value)?));
        }
        Err(Error::InvalidBase58EncodedData { description: value })
    }
}

impl From<ImplicitAddress> for String {
    fn from(value: ImplicitAddress) -> Self {
        match value {
            ImplicitAddress::TZ1(value) => value.into(),
            ImplicitAddress::TZ2(value) => value.into(),
            ImplicitAddress::TZ3(value) => value.into(),
        }
    }
}

impl TryFrom<&Vec<u8>> for ImplicitAddress {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        Self::from_bytes(value)
    }
}

impl TryFrom<String> for ImplicitAddress {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ImplicitAddress::new(value)
    }
}

impl TryFrom<&str> for ImplicitAddress {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        ImplicitAddress::new(value.to_string())
    }
}

impl TryFrom<&ImplicitAddress> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &ImplicitAddress) -> Result<Self> {
        value.to_bytes()
    }
}

impl TryFrom<Address> for ImplicitAddress {
    type Error = Error;

    fn try_from(value: Address) -> Result<Self> {
        if let Address::Implicit(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidAddress)
    }
}

impl From<Ed25519PublicKeyHash> for ImplicitAddress {
    fn from(value: Ed25519PublicKeyHash) -> Self {
        Self::TZ1(value)
    }
}

impl From<Secp256K1PublicKeyHash> for ImplicitAddress {
    fn from(value: Secp256K1PublicKeyHash) -> Self {
        Self::TZ2(value)
    }
}

impl From<P256PublicKeyHash> for ImplicitAddress {
    fn from(value: P256PublicKeyHash) -> Self {
        Self::TZ3(value)
    }
}

/// A base58 encoded contract address with optianally an entrypoint.
///
/// See also: [ContractHash].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String")
)]
pub struct ContractAddress(String);

impl ContractAddress {
    /// Returns only the base58 encoded portion of the contract address without the entrypoint.
    pub fn contract_hash(&self) -> &str {
        let (address, _) = Self::split_to_components(&self.0).unwrap();
        address
    }

    /// Returns only the entrypoint if any.
    pub fn entrypoint(&self) -> Option<&str> {
        let (_, entrypoint) = Self::split_to_components(&self.0).unwrap();
        entrypoint
    }

    /// Creates a `ContractAddress` from a `ContractHash` and an entrypoint.
    pub fn from_components(contract_hash: &ContractHash, entrypoint: Option<&str>) -> Self {
        let tail = entrypoint
            .map(|entrypoint| format!("%{}", entrypoint))
            .unwrap_or("".into());
        let value = format!("{}{}", contract_hash.value(), tail);
        Self(value)
    }

    /// Returns true if the provided value is a valid contract address, false otherwise.
    pub fn is_valid_base58(value: &str) -> bool {
        if let Ok((value, _)) = Self::split_to_components(value) {
            return ContractHash::is_valid_base58(value);
        }
        false
    }

    /// Returns true if the provided value is a valid contract address, false otherwise.
    pub fn is_valid_bytes(value: &[u8]) -> bool {
        let meta = Self::meta_value();
        let bytes = if value.len() > meta.prefixed_bytes_length() {
            &value[0..meta.bytes_length]
        } else {
            value
        };
        return ContractHash::is_valid_bytes(bytes);
    }

    fn split_to_components(value: &str) -> Result<(&str, Option<&str>)> {
        let components = value.split("%").collect::<Vec<_>>();
        if components.len() > 2 {
            return Err(Error::InvalidContractAddress);
        }
        Ok((
            components[0],
            if components.len() == 2 {
                Some(components[1])
            } else {
                None
            },
        ))
    }
}

impl Encoded for ContractAddress {
    type Coder = ContractAddressBytesCoder;

    fn value(&self) -> &str {
        &self.0
    }

    fn meta(&self) -> &'static MetaEncoded {
        ContractHash::meta_value()
    }

    fn new(value: String) -> Result<Self> {
        let (address, _) = Self::split_to_components(value.as_str())?;
        if !ContractHash::is_valid_base58(address) {
            return Err(Error::InvalidBase58EncodedData {
                description: address.into(),
            });
        }
        Ok(Self(value))
    }
}

impl TraitMetaEncoded for ContractAddress {
    fn meta_value() -> &'static MetaEncoded {
        ContractHash::meta_value()
    }
}

impl From<ContractAddress> for String {
    fn from(value: ContractAddress) -> Self {
        value.0
    }
}

impl TryFrom<&Vec<u8>> for ContractAddress {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        Self::from_bytes(value)
    }
}

impl TryFrom<String> for ContractAddress {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ContractAddress {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.into())
    }
}

impl TryFrom<&ContractAddress> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &ContractAddress) -> Result<Self> {
        value.to_bytes()
    }
}

impl From<ContractAddress> for ContractHash {
    fn from(value: ContractAddress) -> Self {
        value.contract_hash().try_into().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tz1_address() -> Result<()> {
        let address: Address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT".try_into()?;
        if let Address::Implicit(value) = address {
            assert_eq!(value.value(), "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_tz1_implicit_address() -> Result<()> {
        let address: Address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT".try_into()?;
        if let Address::Implicit(ImplicitAddress::TZ1(value)) = address {
            assert_eq!(value.value(), "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_kt1_address() -> Result<()> {
        let address: Address = "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo".try_into()?;
        if let Address::Originated(value) = address {
            assert_eq!(value.value(), "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_kt1_address_to_string() -> Result<()> {
        let address: Address = "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo".try_into()?;
        assert_eq!(
            String::from(address),
            "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo"
        );
        return Ok(());
    }

    #[test]
    fn test_tz2_address() -> Result<()> {
        let address: Address = "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot".try_into()?;
        if let Address::Implicit(value) = address {
            assert_eq!(value.value(), "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_tz2_implicit_address() -> Result<()> {
        let address: Address = "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot".try_into()?;
        if let Address::Implicit(ImplicitAddress::TZ2(value)) = address {
            assert_eq!(value.value(), "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_tz3_address() -> Result<()> {
        let address: Address = "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q".try_into()?;
        if let Address::Implicit(value) = address {
            assert_eq!(value.value(), "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_tz3_implicit_address() -> Result<()> {
        let address: Address = "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q".try_into()?;
        if let Address::Implicit(ImplicitAddress::TZ3(value)) = address {
            assert_eq!(value.value(), "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q");
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }
}
