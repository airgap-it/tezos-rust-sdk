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
        let signature = operation.signature().to_bytes()?;
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
