use tezos_core::{
    internal::crypto::Crypto,
    types::encoded::{
        Ed25519PublicKey, Ed25519SecretKey, Ed25519Signature, Encoded, P256PublicKey,
        P256SecretKey, P256Signature, PublicKey, Secp256K1PublicKey, Secp256K1SecretKey,
        Secp256K1Signature, SecretKey, Signature,
    },
};

use crate::{
    operations::{Operation, SignedOperation, UnsignedOperation},
    Error, Result,
};

pub trait Signer<Secret>
where
    Secret: ?Sized,
{
    type Message: ?Sized;
    type Output;
    type Error;

    fn sign(
        &self,
        message: &Self::Message,
        secret: &Secret,
    ) -> std::result::Result<Self::Output, Self::Error>;
}

pub trait Verifier<Public>
where
    Public: ?Sized,
{
    type Message: ?Sized;

    fn verify(&self, message: &Self::Message, key: &Public) -> Result<bool>;
}

pub struct OperationSigner {
    crypto: Crypto,
}

impl OperationSigner {
    const WATERMARK: u8 = 3;
    const MESSAGE_HASH_SIZE: usize = 32;

    pub fn new(crypto: Crypto) -> Self {
        Self { crypto }
    }

    fn sign_raw<F>(&self, operation: &UnsignedOperation, key: &[u8], signer: F) -> Result<Vec<u8>>
    where
        F: FnOnce(&[u8], &[u8]) -> Result<Vec<u8>>,
    {
        signer(&self.hash(operation)?, key)
    }

    fn hash<O: Operation>(&self, operation: &O) -> Result<Vec<u8>> {
        Ok(self.crypto.blake2b(
            &[&[Self::WATERMARK], operation.to_forged_bytes()?.as_slice()].concat(),
            Self::MESSAGE_HASH_SIZE,
        )?)
    }

    fn verify_raw<F>(&self, operation: &SignedOperation, key: &[u8], verifier: F) -> Result<bool>
    where
        F: FnOnce(&[u8], &[u8], &[u8]) -> Result<bool>,
    {
        let signature = operation.signature.to_bytes()?;
        self.hash(operation)
            .map_or(Ok(false), |message| verifier(&message, &signature, key))
    }
}

impl Signer<SecretKey> for OperationSigner {
    type Message = UnsignedOperation;
    type Output = Signature;
    type Error = Error;

    fn sign(&self, message: &Self::Message, secret: &SecretKey) -> Result<Self::Output> {
        match secret {
            SecretKey::Ed25519(key) => self.sign(message, key).map(|signature| signature.into()),
            SecretKey::Secp256K1(key) => self.sign(message, key).map(|signature| signature.into()),
            SecretKey::P256(key) => self.sign(message, key).map(|signature| signature.into()),
        }
    }
}

impl Verifier<PublicKey> for OperationSigner {
    type Message = SignedOperation;

    fn verify(&self, message: &Self::Message, key: &PublicKey) -> Result<bool> {
        match key {
            PublicKey::Ed25519(key) => self.verify(message, key),
            PublicKey::Secp256K1(key) => self.verify(message, key),
            PublicKey::P256(key) => self.verify(message, key),
        }
    }
}

impl Signer<Ed25519SecretKey> for OperationSigner {
    type Message = UnsignedOperation;
    type Output = Ed25519Signature;
    type Error = Error;

    fn sign(&self, message: &Self::Message, secret: &Ed25519SecretKey) -> Result<Self::Output> {
        let key = secret.to_bytes()?;
        let signature = self.sign_raw(message, &key, |message, secret| {
            Ok(self.crypto.sign_ed25519(message, secret)?)
        })?;

        Ok((&signature).try_into()?)
    }
}

impl Verifier<Ed25519PublicKey> for OperationSigner {
    type Message = SignedOperation;

    fn verify(&self, message: &Self::Message, key: &Ed25519PublicKey) -> Result<bool> {
        let key = key.to_bytes()?;
        self.verify_raw(message, &key, |message, signature, key| {
            Ok(self.crypto.verify_ed25519(message, signature, key)?)
        })
    }
}

impl Signer<Secp256K1SecretKey> for OperationSigner {
    type Message = UnsignedOperation;
    type Output = Secp256K1Signature;
    type Error = Error;

    fn sign(&self, message: &Self::Message, secret: &Secp256K1SecretKey) -> Result<Self::Output> {
        let key = secret.to_bytes()?;
        let signature = self.sign_raw(message, &key, |message, secret| {
            Ok(self.crypto.sign_secp256_k1(message, secret)?)
        })?;

        Ok((&signature).try_into()?)
    }
}

impl Verifier<Secp256K1PublicKey> for OperationSigner {
    type Message = SignedOperation;

    fn verify(&self, message: &Self::Message, key: &Secp256K1PublicKey) -> Result<bool> {
        let key = key.to_bytes()?;
        self.verify_raw(message, &key, |message, signature, key| {
            Ok(self.crypto.verify_secp256_k1(message, signature, key)?)
        })
    }
}

impl Signer<P256SecretKey> for OperationSigner {
    type Message = UnsignedOperation;
    type Output = P256Signature;
    type Error = Error;

    fn sign(&self, message: &Self::Message, secret: &P256SecretKey) -> Result<Self::Output> {
        let key = secret.to_bytes()?;
        let signature = self.sign_raw(message, &key, |message, secret| {
            Ok(self.crypto.sign_p256(message, secret)?)
        })?;

        Ok((&signature).try_into()?)
    }
}

impl Verifier<P256PublicKey> for OperationSigner {
    type Message = SignedOperation;

    fn verify(&self, message: &Self::Message, key: &P256PublicKey) -> Result<bool> {
        let key = key.to_bytes()?;
        self.verify_raw(message, &key, |message, signature, key| {
            Ok(self.crypto.verify_p256(message, signature, key)?)
        })
    }
}

#[cfg(test)]
mod test {
    #[cfg(any(feature = "ed25519", feature = "secp256_k1", feature = "p256"))]
    use crate::operations::SeedNonceRevelation;

    use super::*;

    #[test]
    fn test_sign() -> Result<()> {
        for ((key, _), operations) in operations_with_signatures() {
            for (operation, signature) in operations {
                let signed = operation.into_signed_operation(&key)?;

                assert_eq!(&signature, &signed.signature);
            }
        }

        Ok(())
    }

    #[test]
    fn test_verify() -> Result<()> {
        for ((_, key), operations) in operations_with_signatures() {
            for (operation, signature) in operations {
                let signed = SignedOperation::from(operation, signature);

                assert!(signed.verify(&key)?);
            }
        }

        Ok(())
    }

    #[cfg(feature = "ed25519")]
    fn ed25519_key_pair() -> (SecretKey, PublicKey) {
        (
            "edskRv7VyXGVZb8EsrR7D9XKUbbAQNQGtALP6QeB16ZCD7SmmJpzyeneJVg3Mq56YLbxRA1kSdAXiswwPiaVfR3NHGMCXCziuZ".try_into().unwrap(),
            "edpkttZKC51wemRqL2QxwpMnEKxWnbd35pq47Y6xsCHp5M1f7LN8NP".try_into().unwrap(),
        )
    }

    #[cfg(feature = "secp256_k1")]
    fn secp256_k1_pair() -> (SecretKey, PublicKey) {
        (
            "spsk1SsrWCpufeXkNruaG9L3Mf9dRyd4D8HsM8ftqseN1fne3x9LNk"
                .try_into()
                .unwrap(),
            "sppk7ZpH5qAjTDZn1o1TW7z2QbQZUcMHRn2wtV4rRfz15eLQrvPkt6k"
                .try_into()
                .unwrap(),
        )
    }

    #[cfg(feature = "p256")]
    fn p256_pair() -> (SecretKey, PublicKey) {
        (
            "p2sk2rVhhi5EfEdhJ3wQGsdc4ZEN3i7Z8f73Bn1xp1JKjETNyJ85oW"
                .try_into()
                .unwrap(),
            "p2pk67fo5oy6byruqDtzVixbM7L3cVBDRMcFhA33XD5w2HF4fRXDJhw"
                .try_into()
                .unwrap(),
        )
    }

    fn operations_with_signatures(
    ) -> Vec<((SecretKey, PublicKey), Vec<(UnsignedOperation, Signature)>)> {
        vec![
            #[cfg(feature = "ed25519")]
            (
                ed25519_key_pair(),
                vec![
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![]), 
                        "edsigtfLuR4pGGfJwYgWZbWi9JGzjLA8ThhThxqFGC8V6u4WTdS4fM7VFQKoN9jPDLKiAW75PtG1bykpnRa6ozr8m12iKGYCxNd".try_into().unwrap()
                    ),
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![
                            SeedNonceRevelation::new(1, "6cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214".try_into().unwrap()).into()
                        ]),
                        "edsigtyP4ZD5NtBBkAkrmXQZg84xt9uCiHBpjqZj2HE65d4V9dkDapSVJ6jvaA4gEEgksVJzqSxdv2rnMyBzPoAfBQwNEqt8Y1x".try_into().unwrap()
                    ),
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![
                            SeedNonceRevelation::new(1, "9d15bcdc0194b327d3cb0dcd05242bc6ff1635da635e38ed7a62b8c413ce6833".try_into().unwrap()).into(),
                            SeedNonceRevelation::new(2, "921ed0115c7cc1b5dcd07ad66ce4d9b2b0186c93c27a80d70b66b4e309add170".try_into().unwrap()).into()
                        ]),
                        "edsigu5i46oiR9Ye45rUJnPNLkEWkLvvGG5uzHCzPuoNFemNAguHBFn5hXiBivnHHdSzGqsMBc8c5cxAUr8Ue6FUVufbM3hECdU".try_into().unwrap()
                    ),
                ]
            ),
            #[cfg(feature = "secp256_k1")]
            (
                secp256_k1_pair(),
                vec![
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![]),
                        "spsig1LLFq38AD2eLN6qqVSFsTRsG2UP5JGj87EgiVoiDDmaoCghVButYNbP8RoqrJqq8hCeacJY2hKx5zfm4QpmQKytZYeKjNw".try_into().unwrap(), // "spsig1LPnrCkaRypLUz3UYdxQGVpxfSAxWwSV2HpaitKWvqRN6CDqqLJwWNn1S9kEWT2ZLrWq7m2361YVMN4LNkc9FVPdxBjYZi".try_into().unwrap(),
                    ),
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![
                            SeedNonceRevelation::new(1, "6cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214".try_into().unwrap()).into()
                        ]),
                        "spsig1LivvNV6aDUqUNyVwRS9xSEQfzoPkwUZqXEhT8tpeGFEHvUVoPCfVDSoKabUwqc7ERKVwZFtjXKGSHXpNxwKkhxxqsL9iT".try_into().unwrap(), // "spsig1SC5sFkHG4YssRxQJQ5onZ8GNvfQDqk5cz1e6fdPhCNva3baoPCiE9fk6JcyUedEDFAEeMBgC7L6LeYBhFHpVrxjs96iuB".try_into().unwrap(),
                    ),
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![
                            SeedNonceRevelation::new(1, "9d15bcdc0194b327d3cb0dcd05242bc6ff1635da635e38ed7a62b8c413ce6833".try_into().unwrap()).into(),
                            SeedNonceRevelation::new(2, "921ed0115c7cc1b5dcd07ad66ce4d9b2b0186c93c27a80d70b66b4e309add170".try_into().unwrap()).into()
                        ]),
                        "spsig1J84hkDfZ216y94k94GwqnrEADm1KHTz5SFmrSRfBC6hfkpxwRw29nAa1baGGwmAdDNwUyrQsWFqJXdw1t3isd28U8AMAu".try_into().unwrap(), // "spsig1XFTLzrozPJ7Kc9aVNwK4hjpub7cWu8a95LmSKNucsPZjrgq3QRcQWtvo1fbBzpeWPK56XaUiJRN6B59kzueT6LCqTWK8R".try_into().unwrap(),
                    ),
                ]
            ),
            #[cfg(feature = "p256")]
            (
                p256_pair(),
                vec![
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![]),
                        "p2sigeAUuv22uDfL2PacEnGPMATgYiqJFGS7iYvXnG19cQHa75Ak8ie5LJKZrFKRqiNHsu31ga1Ymn2h2d2oJfFNqDPuBSYAH4".try_into().unwrap(), // "p2sigY5tNCTjyR3w2rbgBHnkcEChmtk43Gt6BKqwX2TsNdpVojk3QgRy9Wf3TMkAyRnagy4LrhC4AfVDFBQK87sqBipsNkCt5N".try_into().unwrap(),
                    ),
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![
                            SeedNonceRevelation::new(1, "6cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214".try_into().unwrap()).into()
                        ]),
                        "p2sigTtg4JV68veqnEXLTvbnc6snBNkDt417QpF7HPVwKQYswhCaD1hLBn1RDTHEgU8eKGD8sEz1g36y6wyoUdknJH4EwAKKdr".try_into().unwrap(), // "p2sigUMZVy7WyyvYawCt8oW4eMvXCTWtmU6PCfsTbKmAUXuHFCcH8ER7ZwtNqsnwYER9DRKXfao9xhUFfYdxZPFFDi4J7nckvt".try_into().unwrap(),
                    ),
                    (
                        UnsignedOperation::new("BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(), vec![
                            SeedNonceRevelation::new(1, "9d15bcdc0194b327d3cb0dcd05242bc6ff1635da635e38ed7a62b8c413ce6833".try_into().unwrap()).into(),
                            SeedNonceRevelation::new(2, "921ed0115c7cc1b5dcd07ad66ce4d9b2b0186c93c27a80d70b66b4e309add170".try_into().unwrap()).into()
                        ]),
                        "p2sigPCKxgbzaH1zxX8Hb4cFvTpUUK2rbESVSPbvgcVSUmzfP8Q3kLYwNkATj2bJrDdPEAn8xqSWq6pia3Sidb2LQzC2DkSTVZ".try_into().unwrap(), // "p2sigrjm1STjRF4ygPiPzd4L34MzCErExERsH79jWwJTdYqdaYbYA29UfE1y8f78268B2xNdT3gzR5tXR7G21DCYyYkGnFe3Dm".try_into().unwrap(),
                    ),
                ]
            )
        ]
    }
}
