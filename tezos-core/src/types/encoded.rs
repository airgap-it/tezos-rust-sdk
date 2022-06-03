mod address;
mod block_hash;
mod block_metadata_hash;
mod block_payload_hash;
mod chain_id;
mod context_hash;
mod contract_hash;
mod cryptobox_public_key_hash;
mod ed25519_blinded_public_key_hash;
mod ed25519_encrypted_seed;
mod ed25519_public_key;
mod ed25519_public_key_hash;
mod ed25519_secret_key;
mod ed25519_seed;
mod ed25519_signature;
mod generic_signature;
mod key;
mod nonce_hash;
mod operation_hash;
mod operation_list_hash;
mod operation_list_list_hash;
mod operation_metadata_hash;
mod operation_metadata_list_hash;
mod operation_metadata_list_list_hash;
mod p256_encrypted_secret_key;
mod p256_public_key;
mod p256_public_key_hash;
mod p256_secret_key;
mod p256_signature;
mod protocol_hash;
mod sapling_address;
mod sapling_spending_key;
mod script_expr_hash;
mod secp256_k1_encrypted_scalar;
mod secp256_k1_encrypted_secret_key;
mod secp256_k1_public_key;
mod secp256_k1_public_key_hash;
mod secp256_k1_scalar;
mod secp256_k1_secret_key;
mod secp256_k1_signature;
mod signature;

pub use self::{
    address::Address,
    address::ImplicitAddress,
    block_hash::{BlockHash, META as META_BLOCK_HASH},
    block_metadata_hash::{BlockMetadataHash, META as META_BLOCK_METADATA_HASH},
    block_payload_hash::{BlockPayloadHash, META as META_BLOCK_PAYLOAD_HASH},
    chain_id::{ChainID, META as META_CHAIN_ID},
    context_hash::{ContextHash, META as META_CONTEXT_HASH},
    contract_hash::{ContractHash, META as META_CONTRACT_HASH},
    cryptobox_public_key_hash::{CryptoboxPublicKeyHash, META as META_CRYPTOBOX_PUBLIC_KEY_HASH},
    ed25519_blinded_public_key_hash::{
        Ed25519BlindedPublicKeyHash, META as META_ED25519_BLINDED_PUBLIC_KEY_HASH,
    },
    ed25519_encrypted_seed::{Ed25519EncryptedSeed, META as META_ED25519_ENCRYPTED_SEED},
    ed25519_public_key::{Ed25519PublicKey, META as META_ED25519_PUBLIC_KEY},
    ed25519_public_key_hash::{Ed25519PublicKeyHash, META as META_ED25519_PUBLIC_KEY_HASH},
    ed25519_secret_key::{Ed25519SecretKey, META as META_ED25519_SECRET_KEY},
    ed25519_seed::{Ed25519Seed, META as META_ED25519_SEED},
    ed25519_signature::{Ed25519Signature, META as META_ED25519_SIGNATURE},
    generic_signature::{GenericSignature, META as META_GENERIC_SIGNATURE},
    key::{Key, PublicKey, SecretKey},
    nonce_hash::{NonceHash, META as META_NONCE_HASH},
    operation_hash::{OperationHash, META as META_OPERATION_HASH},
    operation_list_hash::{OperationListHash, META as META_OPERATION_LIST_HASH},
    operation_list_list_hash::{OperationListListHash, META as META_OPERATION_LIST_LIST_HASH},
    operation_metadata_hash::{OperationMetadataHash, META as META_OPERATION_METADATA_HASH},
    operation_metadata_list_hash::{
        OperationMetadataListHash, META as META_OPERATION_METADATA_LIST_HASH,
    },
    operation_metadata_list_list_hash::{
        OperationMetadataListListHash, META as META_OPERATION_METADATA_LIST_LIST_HASH,
    },
    p256_encrypted_secret_key::{P256EncryptedSecretKey, META as META_P256_ENCRYPTED_SECRET_KEY},
    p256_public_key::{P256PublicKey, META as META_P256_PUBLIC_KEY},
    p256_public_key_hash::{P256PublicKeyHash, META as META_P256_PUBLIC_KEY_HASH},
    p256_secret_key::{P256SecretKey, META as META_P256_SECRET_KEY},
    p256_signature::{P256Signature, META as META_P256_SIGNATURE},
    protocol_hash::{ProtocolHash, META as META_PROTOCOL_HASH},
    sapling_address::{SaplingAddress, META as META_SAPLING_ADDRESS},
    sapling_spending_key::{SaplingSpendingKey, META as META_SAPLING_SPENDING_KEY},
    script_expr_hash::{ScriptExprHash, META as META_SCRIPT_EXPR_HASH},
    secp256_k1_encrypted_scalar::{
        Secp256K1EncryptedScalar, META as META_SECP256_K1_ENCRYPTED_SCALAR,
    },
    secp256_k1_encrypted_secret_key::{
        Secp256K1EncryptedSecretKey, META as META_SECP256_K1_ENCRYPTED_SECRET_KEY,
    },
    secp256_k1_public_key::{Secp256K1PublicKey, META as META_SECP256_K1_PUBLIC_KEY},
    secp256_k1_public_key_hash::{Secp256K1PublicKeyHash, META as META_SECP256_K1_PUBLIC_KEY_HASH},
    secp256_k1_scalar::{Secp256K1Scalar, META as META_SECP256_K1_SCALAR},
    secp256_k1_secret_key::{Secp256K1SecretKey, META as META_SECP256_K1_SECRET_KEY},
    secp256_k1_signature::{Secp256K1Signature, META as META_SECP256_K1_SIGNATURE},
};

use crate::{
    internal::coder::{encoded::encoded_bytes_coder::EncodedBytesCoder, Encoder},
    Error, Result,
};

pub trait Encoded: Sized {
    fn base58(&self) -> &str;
    fn meta(&self) -> &MetaEncoded;
    fn new(base58: String) -> Result<Self>;

    fn to_string(&self) -> String {
        self.base58().to_string()
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let coder = EncodedBytesCoder::new();
        coder.encode(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaEncoded {
    pub base58_prefix: &'static str,
    pub base58_length: usize,
    bytes_prefix: &'static [u8],
    pub bytes_length: usize,
}

impl MetaEncoded {
    pub fn version(&self) -> u8 {
        self.bytes_prefix[0]
    }

    pub fn bytes_prefix(&self) -> &'static [u8] {
        &self.bytes_prefix[1..]
    }

    pub fn versioned_bytes_prefix(&self) -> &'static [u8] {
        self.bytes_prefix
    }

    pub fn is_valid_base58(&self, value: &str) -> bool {
        value.starts_with(self.base58_prefix) && value.len() == self.base58_length
    }

    pub fn is_valid_bytes(&self, value: &[u8]) -> bool {
        value.len() == self.bytes_length || self.is_valid_prefixed_bytes(value)
    }

    pub fn is_valid_prefixed_bytes(&self, value: &[u8]) -> bool {
        value.starts_with(&self.versioned_bytes_prefix())
            && value.len() == (self.bytes_length + self.versioned_bytes_prefix().len())
    }

    pub fn is_valid_consumable_bytes(&self, value: &[u8]) -> bool {
        value.len() == self.bytes_length || self.is_valid_prefixed_consumable_bytes(value)
    }

    pub fn is_valid_prefixed_consumable_bytes(&self, value: &[u8]) -> bool {
        value.starts_with(&self.versioned_bytes_prefix())
            && value.len() >= (self.bytes_length + self.versioned_bytes_prefix().len())
    }

    pub fn recognize_base58(value: &str) -> Result<&'static MetaEncoded> {
        META_ENCODED_VALUES
            .iter()
            .find(|item| item.is_valid_base58(value))
            .map(|item| *item)
            .ok_or(Error::InvalidBase58EncodedData)
    }

    pub fn recognize_bytes(value: &[u8]) -> Result<&'static MetaEncoded> {
        META_ENCODED_VALUES
            .iter()
            .find(|item| item.is_valid_prefixed_bytes(value))
            .map(|item| *item)
            .ok_or(Error::InvalidBase58EncodedData)
    }

    pub fn recognize_consumable_bytes(value: &[u8]) -> Result<&'static MetaEncoded> {
        META_ENCODED_VALUES
            .iter()
            .find(|item| item.is_valid_prefixed_consumable_bytes(value))
            .map(|item| *item)
            .ok_or(Error::InvalidBase58EncodedData)
    }
}

const META_ENCODED_VALUES: &[&'static MetaEncoded] = &[
    &block_hash::META,
    &block_metadata_hash::META,
    &block_payload_hash::META,
    &chain_id::META,
    &context_hash::META,
    &cryptobox_public_key_hash::META,
    &ed25519_blinded_public_key_hash::META,
    &ed25519_encrypted_seed::META,
    &ed25519_public_key::META,
    &ed25519_public_key_hash::META,
    &ed25519_secret_key::META,
    &ed25519_seed::META,
    &ed25519_signature::META,
    &generic_signature::META,
    &nonce_hash::META,
    &operation_hash::META,
    &operation_list_hash::META,
    &operation_list_list_hash::META,
    &operation_metadata_hash::META,
    &operation_metadata_list_hash::META,
    &operation_metadata_list_list_hash::META,
    &p256_encrypted_secret_key::META,
    &p256_public_key::META,
    &p256_public_key_hash::META,
    &p256_secret_key::META,
    &p256_signature::META,
    &protocol_hash::META,
    &sapling_address::META,
    &sapling_spending_key::META,
    &script_expr_hash::META,
    &secp256_k1_encrypted_scalar::META,
    &secp256_k1_encrypted_secret_key::META,
    &secp256_k1_public_key::META,
    &secp256_k1_public_key_hash::META,
    &secp256_k1_scalar::META,
    &secp256_k1_secret_key::META,
    &secp256_k1_signature::META,
];
