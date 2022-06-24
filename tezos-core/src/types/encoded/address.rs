use super::{
    contract_hash::ContractHash, ed25519_public_key_hash::Ed25519PublicKeyHash,
    p256_public_key_hash::P256PublicKeyHash, secp256_k1_public_key_hash::Secp256K1PublicKeyHash,
    Encoded, MetaEncoded,
};
use crate::{
    internal::coder::{AddressBytesCoder, ContractAddressBytesCoder, ImplicitAddressBytesCoder},
    Error, Result,
};

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
        Err(Error::InvalidBase58EncodedData)
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
        Err(Error::InvalidBase58EncodedData)
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

pub struct ContractAddress {
    contract_hash: ContractHash,
    entrypoint: Option<String>,
}

impl ContractAddress {
    pub fn entrypoint(&self) -> Option<&str> {
        self.entrypoint.as_ref().map(|value| &**value)
    }

    pub fn from_components(contract_hash: ContractHash, entrypoint: Option<String>) -> Self {
        Self {
            contract_hash,
            entrypoint,
        }
    }

    pub fn is_valid_base58(value: &str) -> bool {
        if let Ok((value, _)) = Self::split_to_components(value) {
            return ContractHash::is_valid_base58(value);
        }
        false
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
        self.contract_hash.value()
    }

    fn meta(&self) -> &'static MetaEncoded {
        self.contract_hash.meta()
    }

    fn new(value: String) -> Result<Self> {
        let (address, entrypoint) = Self::split_to_components(value.as_str())?;
        Ok(Self {
            contract_hash: ContractHash::new(address.into())?,
            entrypoint: entrypoint.map(|value| value.into()),
        })
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
