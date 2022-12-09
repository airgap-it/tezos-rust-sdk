//! All the Tezos basic types that can be encoded/decoded to base58 bytes.

mod address;
mod key;
mod macros;
mod meta_encoded;
mod signature;

pub use self::{
    address::{Address, ContractAddress, ImplicitAddress},
    key::{Key, PublicKey, SecretKey},
    meta_encoded::{MetaEncoded, TraitMetaEncoded},
    signature::Signature,
};

use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::ConsumableList,
    },
    Error, Result,
};

use macros::{make_encoded_struct, make_encoded_structs};

/// Trait implemented by all the values that can be represented as base58 encoded strings.
pub trait Encoded: Sized {
    /// The type used to encode and decode the value.
    type Coder: Encoder<Self, Vec<u8>, Error> + Decoder<Self, [u8], Error>;
    /// Returns the base58 string slice value.
    fn value(&self) -> &str;
    /// Returns the metadata.
    fn meta(&self) -> &'static MetaEncoded;
    /// Creates a new instance for the given value.
    ///
    /// Returns an error of the provided value is not a valid base58 string for the type being created.
    fn new(value: String) -> Result<Self>;
    /// Returns the base58 string value.
    fn into_string(&self) -> String {
        self.value().into()
    }
    /// Encodes the value to its bytes representation
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Self::Coder::encode(self)
    }
    /// Creates an instance from the given bytes.
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::Coder::decode(bytes)
    }
    /// Creates an instance from the given consumable bytes.
    fn from_consumable_bytes<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Self>
    where
        Self::Coder: ConsumingDecoder<Self, u8, Error>,
    {
        Self::Coder::decode_consuming(bytes)
    }
}

make_encoded_structs!(
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct block_hash::BlockHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "B",
            base58_length: 51,
            bytes_prefix: [1, 52,],
            bytes_length: 32,
        }
        test {
            string_value: "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA",
            bytes_value: [
                153, 103, 160, 148, 80, 29, 14, 26, 87, 234, 129, 153, 155, 172, 15, 6, 24, 44, 32, 47,
                78, 64, 97, 80, 51, 203, 69, 223, 229, 241, 173, 76,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct block_metadata_hash::BlockMetadataHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "bm",
            base58_length: 52,
            bytes_prefix: [234, 249,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct block_payload_hash::BlockPayloadHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "vh",
            base58_length: 52,
            bytes_prefix: [1, 106, 242,],
            bytes_length: 32,
        }
        test {
            string_value: "vh3ZMFvh79oP7WiYs4kUdirPE2UGVFDJPzWPHQWHic3zsTJurwTU",
            bytes_value: [
                248, 3, 45, 179, 210, 181, 30, 158, 205, 50, 116, 10, 226, 52, 91, 157, 117, 205,
                238, 139, 146, 126, 105, 146, 174, 25, 28, 209, 238, 87, 4, 173,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct chain_id::ChainId;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "Net",
            base58_length: 15,
            bytes_prefix: [87, 82, 0,],
            bytes_length: 4,
        }
        test {
            string_value: "NetXdQprcVkpaWU",
            bytes_value: [122, 6, 167, 112,],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct context_hash::ContextHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "Co",
            base58_length: 52,
            bytes_prefix: [79, 199,],
            bytes_length: 32,
        }
        test {
            string_value: "CoUiATua7N2jitdscnVnqDpmfnqwwiJyCZbco6qfmcykVmGwPLbY",
            bytes_value: [
                8, 193, 4, 139, 153, 132, 175, 196, 86, 32, 232, 16, 131, 217, 241, 254, 44, 4,
                138, 199, 118, 31, 137, 238, 8, 51, 223, 86, 180, 53, 103, 2,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct contract_hash::ContractHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "KT1",
            base58_length: 36,
            bytes_prefix: [2, 90, 121,],
            bytes_length: 20,
        }
        test {
            string_value: "KT1QTcAXeefhJ3iXLurRt81WRKdv7YqyYFmo",
            bytes_value: [
                174, 39, 64, 233, 124, 78, 32, 128, 225, 6, 128, 60, 217, 114, 47, 117, 226, 161, 27,
                197,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct cryptobox_public_key_hash::CryptoboxPublicKeyHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "id",
            base58_length: 30,
            bytes_prefix: [153, 103,],
            bytes_length: 16,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct ed25519_blinded_public_key_hash::Ed25519BlindedPublicKeyHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "btz1",
            base58_length: 37,
            bytes_prefix: [1, 2, 49, 223,],
            bytes_length: 20,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct ed25519_encrypted_seed::Ed25519EncryptedSeed;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "edesk",
            base58_length: 88,
            bytes_prefix: [7, 90, 60, 179, 41,],
            bytes_length: 56,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct ed25519_public_key::Ed25519PublicKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "edpk",
            base58_length: 54,
            bytes_prefix: [13, 15, 37, 217,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct ed25519_public_key_hash::Ed25519PublicKeyHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "tz1",
            base58_length: 36,
            bytes_prefix: [6, 161, 159,],
            bytes_length: 20,
        }
        test {
            string_value: "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT",
            bytes_value: [
                22u8, 230, 73, 148, 194, 221, 189, 41, 54, 149, 182, 62, 76, 173, 224, 41, 211, 200,
                181, 227,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct ed25519_secret_key::Ed25519SecretKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "edsk",
            base58_length: 98,
            bytes_prefix: [43, 246, 78, 7,],
            bytes_length: 64,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct ed25519_seed::Ed25519Seed;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "edsk",
            base58_length: 54,
            bytes_prefix: [13, 15, 58, 7,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
            types::encoded::GenericSignature,
        }
        struct ed25519_signature::Ed25519Signature;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "edsig",
            base58_length: 99,
            bytes_prefix: [9, 245, 205, 134, 18,],
            bytes_length: 64,
        }
        extra_try_from {
            &GenericSignature,
        }
        test {
            string_value: "edsigtXomBKi5CTRf5cjATJWSyaRvhfYNHqSUGrn4SdbYRcGwQrUGjzEfQDTuqHhuA8b2d8NarZjz8TRf65WkpQmo423BtomS8Q",
            bytes_value: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
            types::encoded::Ed25519Signature,
            types::encoded::Secp256K1Signature,
            types::encoded::P256Signature,
        }
        struct generic_signature::GenericSignature;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "sig",
            base58_length: 96,
            bytes_prefix: [4, 130, 43,],
            bytes_length: 64,
        }
        extra_try_from {
            &Ed25519Signature,
            &Secp256K1Signature,
            &P256Signature,
        }
        test {
            string_value: "sigS9AYGk12AjCHEPowcEhrURh1Stk2TyqqDEKqGEPLBTeAsgq8ZRgPdDc4TvxMEy8PLBqf4BtDptD4jY5o9yjJzSqpeqPpS",
            bytes_value: [
                31, 190, 78, 29, 238, 27, 255, 115, 99, 209, 110, 112, 35, 100, 193, 115, 218, 145,
                132, 232, 30, 75, 195, 172, 85, 230, 127, 133, 215, 71, 44, 244, 12, 23, 159, 110, 63,
                239, 226, 87, 158, 196, 71, 85, 230, 88, 73, 110, 127, 78, 130, 222, 143, 88, 55, 85,
                254, 62, 242, 157, 247, 151, 88, 6,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct nonce_hash::NonceHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "nce",
            base58_length: 53,
            bytes_prefix: [69, 220, 169,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct operation_hash::OperationHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "o",
            base58_length: 51,
            bytes_prefix: [5, 116,],
            bytes_length: 32,
        }
        test {
            string_value: "ooG169iWhv7vQccPGcB2EWeAjFWvxcrmQVCi4eWCviUTHeQuH24",
            bytes_value: [
                81, 67, 247, 201, 125, 8, 189, 62, 40, 197, 46, 38, 67, 145, 144, 95, 233, 123, 38,
                150, 214, 84, 97, 115, 22, 163, 84, 51, 118, 106, 80, 7,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct operation_list_hash::OperationListHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "Lo",
            base58_length: 52,
            bytes_prefix: [133, 233,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct operation_list_list_hash::OperationListListHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "LLo",
            base58_length: 53,
            bytes_prefix: [29, 159, 109,],
            bytes_length: 32,
        }
        test {
            string_value: "LLoZpN9vikYaszkBgE5dELmghpyskaXjhwDzUQ9zNX5ou2qXYsd4r",
            bytes_value: [
                65, 18, 20, 6, 159, 96, 252, 51, 234, 134, 99, 127, 244, 54, 69, 93, 187, 225, 63, 110,
                148, 143, 4, 247, 186, 156, 179, 75, 96, 67, 210, 150,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct operation_metadata_hash::OperationMetadataHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "r",
            base58_length: 51,
            bytes_prefix: [5, 183,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct operation_metadata_list_hash::OperationMetadataListHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "Lr",
            base58_length: 52,
            bytes_prefix: [134, 39,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct operation_metadata_list_list_hash::OperationMetadataListListHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "LLr",
            base58_length: 53,
            bytes_prefix: [29, 159, 182,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct p256_encrypted_secret_key::P256EncryptedSecretKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "p2esk",
            base58_length: 88,
            bytes_prefix: [9, 48, 57, 115, 171,],
            bytes_length: 56,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct p256_public_key::P256PublicKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "p2pk",
            base58_length: 55,
            bytes_prefix: [3, 178, 139, 127,],
            bytes_length: 33,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct p256_public_key_hash::P256PublicKeyHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "tz3",
            base58_length: 36,
            bytes_prefix: [6, 161, 164,],
            bytes_length: 20,
        }
        test {
            string_value: "tz3hw2kqXhLUvY65ca1eety2oQTpAvd34R9Q",
            bytes_value: [
                236, 248, 123, 167, 175, 44, 31, 221, 193, 241, 39, 219, 102, 192, 143, 101, 63, 141,
                55, 63,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct p256_secret_key::P256SecretKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "p2sk",
            base58_length: 54,
            bytes_prefix: [16, 81, 238, 189,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
            types::encoded::GenericSignature,
        }
        struct p256_signature::P256Signature;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "p2sig",
            base58_length: 98,
            bytes_prefix: [54, 240, 44, 52,],
            bytes_length: 64,
        }
        extra_try_from {
            &GenericSignature,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct protocol_hash::ProtocolHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "P",
            base58_length: 51,
            bytes_prefix: [2, 170,],
            bytes_length: 32,
        }
        test {
            string_value: "Psithaca2MLRFYargivpo7YvUr7wUDqyxrdhC5CQq78mRvimz6A",
            bytes_value: [
                132, 36, 82, 12, 249, 187, 240, 164, 39, 112, 32, 77, 149, 220, 193, 241, 30, 64, 79,
                219, 62, 144, 200, 72, 80, 196, 207, 219, 80, 197, 196, 185,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct sapling_address::SaplingAddress;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "zet1",
            base58_length: 69,
            bytes_prefix: [18, 71, 40, 223,],
            bytes_length: 43,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct sapling_spending_key::SaplingSpendingKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "sask",
            base58_length: 241,
            bytes_prefix: [11, 237, 20, 92,],
            bytes_length: 169,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct script_expr_hash::ScriptExprHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "expr",
            base58_length: 54,
            bytes_prefix: [13, 44, 64, 27,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct secp256_k1_encrypted_scalar::Secp256K1EncryptedScalar;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "seesk",
            base58_length: 93,
            bytes_prefix: [1, 131, 36, 86, 248,],
            bytes_length: 60,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct secp256_k1_encrypted_secret_key::Secp256K1EncryptedSecretKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "spesk",
            base58_length: 88,
            bytes_prefix: [9, 237, 241, 174, 150,],
            bytes_length: 56,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct secp256_k1_public_key::Secp256K1PublicKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "sppk",
            base58_length: 55,
            bytes_prefix: [3, 254, 226, 86,],
            bytes_length: 33,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct secp256_k1_public_key_hash::Secp256K1PublicKeyHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "tz2",
            base58_length: 36,
            bytes_prefix: [6, 161, 161,],
            bytes_length: 20,
        }
        test {
            string_value: "tz2MgpiRm5NB1rpGf5nCURbC11UNrneScoot",
            bytes_value: [
                146, 174, 203, 241, 51, 100, 160, 131, 26, 170, 172, 13, 238, 138, 117, 165, 40, 48,
                171, 55,
            ],
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct secp256_k1_scalar::Secp256K1Scalar;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "SSp",
            base58_length: 53,
            bytes_prefix: [38, 248, 136,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct secp256_k1_secret_key::Secp256K1SecretKey;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "spsk",
            base58_length: 54,
            bytes_prefix: [17, 162, 224, 201,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
            types::encoded::GenericSignature,
        }
        struct secp256_k1_signature::Secp256K1Signature;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "spsig1",
            base58_length: 99,
            bytes_prefix: [13, 115, 101, 19, 63,],
            bytes_length: 64,
        }
        extra_try_from {
            &GenericSignature,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct tx_rollup_id::TxRollupId;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "txr1",
            base58_length: 37,
            bytes_prefix: [1, 128, 120, 31,],
            bytes_length: 20,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct message_result_hash::MessageResultHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "txmr",
            base58_length: 54,
            bytes_prefix: [18, 7, 206, 87,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct message_result_list_hash::MessageResultListHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "txM",
            base58_length: 53,
            bytes_prefix: [79, 146, 82,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct commitment_hash::CommitmentHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "txc",
            base58_length: 53,
            bytes_prefix: [79, 148, 17,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct inbox_hash::InboxHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "txi",
            base58_length: 53,
            bytes_prefix: [79, 148, 196,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct tx_rollup_l2_address::TxRollupL2Address;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "tz4",
            base58_length: 36,
            bytes_prefix: [6, 161, 166,],
            bytes_length: 20,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct withdraw_list_hash::WithdrawListHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "txw",
            base58_length: 53,
            bytes_prefix: [79, 150, 72,],
            bytes_length: 32,
        }
    },
    {
        use {
            internal::coder::EncodedBytesCoder,
        }
        struct sc_rollup_reveal_hash::ScRollupRevealHash;
        coder: EncodedBytesCoder;
        meta {
            base58_prefix: "scrrh1",
            base58_length: 56,
            bytes_prefix: [230, 206, 128, 200, 196,],
            bytes_length: 32,
        }
        test {
            string_value: "scrrh1387HMYSHbCvWaLrRsTP9Ndh68xGEMRb37bh6mZgUn1oyZ5sfbS",
            bytes_value: [
                52, 142, 195, 75, 199, 253, 40, 202, 117, 54, 126, 155, 18, 69, 127, 11,
                31, 31, 206, 127, 26, 125, 36, 5, 88, 47, 191, 145, 24, 83, 28, 27,
            ],
        }
    },
);
