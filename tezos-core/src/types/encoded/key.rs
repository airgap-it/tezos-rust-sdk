use crate::{
    internal::coder::{
        encoded::{
            encoded_bytes_coder::EncodedBytesCoder, public_key_bytes_coder::PublicKeyBytesCoder,
        },
        Decoder, Encoder,
    },
    types::encoded::{
        Ed25519PublicKey, Ed25519SecretKey, Encoded, MetaEncoded, P256PublicKey, P256SecretKey,
        Secp256K1PublicKey, Secp256K1SecretKey,
    },
    Error, Result,
};

#[derive(Debug)]
pub enum Key {
    Secret(SecretKey),
    Public(PublicKey),
}

impl Encoded for Key {
    fn base58(&self) -> &str {
        match self {
            Self::Secret(value) => value.base58(),
            Self::Public(value) => value.base58(),
        }
    }

    fn meta(&self) -> &MetaEncoded {
        match self {
            Self::Secret(value) => value.meta(),
            Self::Public(value) => value.meta(),
        }
    }

    fn new(base58: String) -> Result<Self> {
        if SecretKey::is_valid_base58(&base58) {
            return Ok(Self::Secret(SecretKey::new(base58)?));
        }
        if PublicKey::is_valid_base58(&base58) {
            return Ok(Self::Public(PublicKey::new(base58)?));
        }
        Err(Error::InvalidBase58EncodedData)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            Key::Secret(value) => value.to_bytes(),
            Key::Public(value) => value.to_bytes(),
        }
    }
}

impl TryFrom<&Vec<u8>> for Key {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode(value)
    }
}

impl TryFrom<String> for Key {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Key::new(value)
    }
}

impl TryFrom<&str> for Key {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Key::new(value.to_string())
    }
}

impl TryFrom<&Key> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Key) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[derive(Debug)]
pub enum SecretKey {
    Ed25519(Ed25519SecretKey),
    Secp256K1(Secp256K1SecretKey),
    P256(P256SecretKey),
}

impl SecretKey {
    pub fn is_valid_base58(value: &str) -> bool {
        Ed25519SecretKey::is_valid_base58(value)
            || Secp256K1SecretKey::is_valid_base58(value)
            || P256SecretKey::is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        Ed25519SecretKey::is_valid_bytes(value)
            || Secp256K1SecretKey::is_valid_bytes(value)
            || P256SecretKey::is_valid_bytes(value)
    }
}

impl Encoded for SecretKey {
    fn base58(&self) -> &str {
        match self {
            Self::Ed25519(value) => value.base58(),
            Self::Secp256K1(value) => value.base58(),
            Self::P256(value) => value.base58(),
        }
    }

    fn meta(&self) -> &MetaEncoded {
        match self {
            Self::Ed25519(value) => value.meta(),
            Self::Secp256K1(value) => value.meta(),
            Self::P256(value) => value.meta(),
        }
    }

    fn new(base58: String) -> Result<Self> {
        if Ed25519SecretKey::is_valid_base58(&base58) {
            return Ok(Self::Ed25519(Ed25519SecretKey::new(base58)?));
        }
        if Secp256K1SecretKey::is_valid_base58(&base58) {
            return Ok(Self::Secp256K1(Secp256K1SecretKey::new(base58)?));
        }
        if P256SecretKey::is_valid_base58(&base58) {
            return Ok(Self::P256(P256SecretKey::new(base58)?));
        }
        Err(Error::InvalidBase58EncodedData)
    }
}

impl TryFrom<&Vec<u8>> for SecretKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode(value)
    }
}

impl TryFrom<String> for SecretKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        SecretKey::new(value)
    }
}

impl TryFrom<&str> for SecretKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        SecretKey::new(value.to_string())
    }
}

impl TryFrom<&SecretKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &SecretKey) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[derive(Debug)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKey),
    Secp256K1(Secp256K1PublicKey),
    P256(P256PublicKey),
}

impl PublicKey {
    pub fn is_valid_base58(value: &str) -> bool {
        Ed25519PublicKey::is_valid_base58(value)
            || Secp256K1PublicKey::is_valid_base58(value)
            || P256PublicKey::is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        Ed25519PublicKey::is_valid_bytes(value)
            || Secp256K1PublicKey::is_valid_bytes(value)
            || P256PublicKey::is_valid_bytes(value)
    }
}

impl Encoded for PublicKey {
    fn base58(&self) -> &str {
        match self {
            Self::Ed25519(value) => value.base58(),
            Self::Secp256K1(value) => value.base58(),
            Self::P256(value) => value.base58(),
        }
    }

    fn meta(&self) -> &MetaEncoded {
        match self {
            Self::Ed25519(value) => value.meta(),
            Self::Secp256K1(value) => value.meta(),
            Self::P256(value) => value.meta(),
        }
    }

    fn new(base58: String) -> Result<Self> {
        if Ed25519PublicKey::is_valid_base58(&base58) {
            return Ok(Self::Ed25519(Ed25519PublicKey::new(base58)?));
        }
        if Secp256K1PublicKey::is_valid_base58(&base58) {
            return Ok(Self::Secp256K1(Secp256K1PublicKey::new(base58)?));
        }
        if P256PublicKey::is_valid_base58(&base58) {
            return Ok(Self::P256(P256PublicKey::new(base58)?));
        }
        Err(Error::InvalidBase58EncodedData)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let coder = PublicKeyBytesCoder::new();
        coder.encode(self)
    }
}

impl TryFrom<&Vec<u8>> for PublicKey {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = PublicKeyBytesCoder::new();
        coder.decode(value)
    }
}

impl TryFrom<String> for PublicKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        PublicKey::new(value)
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        PublicKey::new(value.to_string())
    }
}

impl TryFrom<&PublicKey> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &PublicKey) -> Result<Self> {
        value.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edpk_key_1() -> Result<()> {
        let key: Key = "edpkuF5y5V7NNH5xKMCKHHqVDzq6YuUXiPT3FFjA9CGnht6xCgziTe".try_into()?;
        if let Key::Public(key) = key {
            assert_eq!(
                key.base58(),
                "edpkuF5y5V7NNH5xKMCKHHqVDzq6YuUXiPT3FFjA9CGnht6xCgziTe"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_edpk_key_2() -> Result<()> {
        let key: Key = "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR".try_into()?;
        if let Key::Secret(key) = key {
            assert_eq!(
                key.base58(),
                "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_edpk_public_key() -> Result<()> {
        let key: PublicKey = "edpkuF5y5V7NNH5xKMCKHHqVDzq6YuUXiPT3FFjA9CGnht6xCgziTe".try_into()?;
        if let PublicKey::Ed25519(key) = key {
            assert_eq!(
                key.base58(),
                "edpkuF5y5V7NNH5xKMCKHHqVDzq6YuUXiPT3FFjA9CGnht6xCgziTe"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_edpk_secret_key() -> Result<()> {
        let key: SecretKey = "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR".try_into()?;
        if let SecretKey::Ed25519(key) = key {
            assert_eq!(
                key.base58(),
                "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_secp_256_k1_key_1() -> Result<()> {
        let key: Key = "sppkDN74FpFyXiHUe7MZS7rwDzzwb2esc21355LEcSExN67KdNnAfqA".try_into()?;
        if let Key::Public(key) = key {
            assert_eq!(
                key.base58(),
                "sppkDN74FpFyXiHUe7MZS7rwDzzwb2esc21355LEcSExN67KdNnAfqA"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_secp_256_k1_key_2() -> Result<()> {
        let key: Key = "spsk2WUw2TFXQq2CsrNhB7EfFzdhMyNvGoYgD4uGQ6e17MgoRDv1co".try_into()?;
        if let Key::Secret(key) = key {
            assert_eq!(
                key.base58(),
                "spsk2WUw2TFXQq2CsrNhB7EfFzdhMyNvGoYgD4uGQ6e17MgoRDv1co"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_secp_256_k1_public_key() -> Result<()> {
        let key: PublicKey =
            "sppkDN74FpFyXiHUe7MZS7rwDzzwb2esc21355LEcSExN67KdNnAfqA".try_into()?;
        if let PublicKey::Secp256K1(key) = key {
            assert_eq!(
                key.base58(),
                "sppkDN74FpFyXiHUe7MZS7rwDzzwb2esc21355LEcSExN67KdNnAfqA"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_secp_256_k1_secret_key() -> Result<()> {
        let key: SecretKey = "spsk2WUw2TFXQq2CsrNhB7EfFzdhMyNvGoYgD4uGQ6e17MgoRDv1co".try_into()?;
        if let SecretKey::Secp256K1(key) = key {
            assert_eq!(
                key.base58(),
                "spsk2WUw2TFXQq2CsrNhB7EfFzdhMyNvGoYgD4uGQ6e17MgoRDv1co"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_p256_key_1() -> Result<()> {
        let key: Key = "p2pkDkL6thzTwyPjpmMotSqeKy1MAftLrseqTALwBhHwUtXRmFV983f".try_into()?;
        if let Key::Public(key) = key {
            assert_eq!(
                key.base58(),
                "p2pkDkL6thzTwyPjpmMotSqeKy1MAftLrseqTALwBhHwUtXRmFV983f"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_p256_key_2() -> Result<()> {
        let key: Key = "p2sk2Xoduh8dx6B3smV81NMV25cYpZJj7yYWMRARedzyJae8SB9auw".try_into()?;
        if let Key::Secret(key) = key {
            assert_eq!(
                key.base58(),
                "p2sk2Xoduh8dx6B3smV81NMV25cYpZJj7yYWMRARedzyJae8SB9auw"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_p256_public_key() -> Result<()> {
        let key: PublicKey =
            "p2pkDkL6thzTwyPjpmMotSqeKy1MAftLrseqTALwBhHwUtXRmFV983f".try_into()?;
        if let PublicKey::P256(key) = key {
            assert_eq!(
                key.base58(),
                "p2pkDkL6thzTwyPjpmMotSqeKy1MAftLrseqTALwBhHwUtXRmFV983f"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }

    #[test]
    fn test_p256_secret_key() -> Result<()> {
        let key: SecretKey = "p2sk2Xoduh8dx6B3smV81NMV25cYpZJj7yYWMRARedzyJae8SB9auw".try_into()?;
        if let SecretKey::P256(key) = key {
            assert_eq!(
                key.base58(),
                "p2sk2Xoduh8dx6B3smV81NMV25cYpZJj7yYWMRARedzyJae8SB9auw"
            );
            return Ok(());
        }
        Err(Error::InvalidConversion)
    }
}
