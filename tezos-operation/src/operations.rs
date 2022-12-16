mod activate_account;
mod ballot;
mod delegation;
mod double_baking_evidence;
mod double_endorsement_evidence;
mod double_preendorsement_evidence;
mod endorsement;
mod failing_noop;
mod origination;
mod preendorsement;
mod proposals;
mod register_global_constant;
mod reveal;
mod seed_nonce_revelation;
mod set_deposit_limit;
mod transaction;

use num_derive::FromPrimitive;
use tezos_core::{
    internal::{
        coder::{Decoder, Encoder},
        crypto::Crypto,
    },
    types::{
        encoded::{BlockHash, Encoded, OperationHash, PublicKey, SecretKey, Signature},
        mutez::Mutez,
    },
    Tezos,
};

use crate::{
    internal::{
        coder::{
            operation_bytes_coder::OperationBytesCoder,
            operation_content_bytes_coder::OperationContentBytesCoder,
        },
        signer::{OperationSigner, Signer, Verifier},
    },
    Result,
};

pub use self::{
    activate_account::ActivateAccount,
    ballot::{Ballot, Type as BallotType},
    delegation::Delegation,
    double_baking_evidence::DoubleBakingEvidence,
    double_endorsement_evidence::DoubleEndorsementEvidence,
    double_preendorsement_evidence::DoublePreendorsementEvidence,
    endorsement::Endorsement,
    failing_noop::FailingNoop,
    origination::{Origination, Script},
    preendorsement::Preendorsement,
    proposals::Proposals,
    register_global_constant::RegisterGlobalConstant,
    reveal::Reveal,
    seed_nonce_revelation::SeedNonceRevelation,
    set_deposit_limit::SetDepositsLimit,
    traits::{
        OperationConsensusContent as TraitOperationConsensusContent,
        OperationContent as TraitOperationContent,
        OperationManagerContent as TraitOperationManagerContent,
    },
    transaction::{Entrypoint, Parameters, Transaction},
};

/// The [Operation] trait defines common methods for [UnsignedOperation] and [SignedOperation].
pub trait Operation {
    /// Returns the operation's branch.
    fn branch(&self) -> &BlockHash;
    /// Returns the operation's content.
    fn contents(&self) -> &[OperationContent];

    /// Returns the operation forged bytes.
    fn to_forged_bytes(&self) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        OperationBytesCoder::encode(self)
    }
}

/// The [UnsignedOperation] struct represent an unsigned tezos operation that can be signed with a [SecretKey].
///
/// # Example
///
/// ```rust
/// use tezos_operation::operations::UnsignedOperation;
/// use tezos_operation::operations::Delegation;
/// use tezos_core::types::encoded::SecretKey;
///
/// let secret_key: SecretKey = "edskRv7VyXGVZb8EsrR7D9XKUbbAQNQGtALP6QeB16ZCD7SmmJpzyeneJVg3Mq56YLbxRA1kSdAXiswwPiaVfR3NHGMCXCziuZ".try_into().unwrap();
/// let unsigned_operation = UnsignedOperation::new("BLyKu3tnc9NCuiFfCqfeVGPCoZTyW63dYh2XAYxkM7fQYKCqsju".try_into().unwrap(), vec![
///     Delegation::new("tz1YY1LvD6TFH4z74pvxPQXBjAKHE5tB5Q8f".try_into().unwrap(), 1000u32.into(), 1u8.into(), 1000u32.into(), 1000u32.into(), Some("tz1bQMn5xYFbX6geRxqvuAiTywsCtNywawxH".try_into().unwrap())).into(),
/// ]);
/// let signed_operation = unsigned_operation.into_signed_operation(&secret_key);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsignedOperation {
    /// The operation branch.
    pub branch: BlockHash,
    /// The operation contents.
    pub contents: Vec<OperationContent>,
}

impl UnsignedOperation {
    /// Creates a new [UnsignedOperation].
    pub fn new(branch: BlockHash, contents: Vec<OperationContent>) -> Self {
        Self { branch, contents }
    }

    /// Creates a new [UnsignedOperation] from the forged bytes.
    pub fn from_forged_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        OperationBytesCoder::decode(bytes.as_ref())
    }

    /// Creates a [SignedOperation] by signing the [UnsignedOperation] with the given secret key and using
    /// the crypto providers configured in the given [Tezos] instance.
    pub fn into_signed_operation_with(
        self,
        key: &SecretKey,
        tezos: &Tezos,
    ) -> Result<SignedOperation> {
        let signature = self.sign_with(key, &tezos)?;

        Ok(SignedOperation::new(self.branch, self.contents, signature))
    }

    /// Creates a [SignedOperation] by signing the [UnsignedOperation] with the given secret key.
    pub fn into_signed_operation(self, key: &SecretKey) -> Result<SignedOperation> {
        let tezos: Tezos = Default::default();

        self.into_signed_operation_with(key, &tezos)
    }

    /// Creates a [Signature] by signing the [UnsignedOperation] with the given secret key and using
    /// the crypto providers configured in the given [Tezos] instance.
    pub fn sign_with(&self, key: &SecretKey, tezos: &Tezos) -> Result<Signature> {
        let signer = OperationSigner::new(tezos.get_crypto());

        signer.sign(self, key)
    }

    /// Creates a [Signature] by signing the [UnsignedOperation] with the given secret key.
    pub fn sign(&self, key: &SecretKey) -> Result<Signature> {
        let tezos: Tezos = Default::default();

        self.sign_with(key, &tezos)
    }
}

impl Operation for UnsignedOperation {
    fn branch(&self) -> &BlockHash {
        &self.branch
    }

    fn contents(&self) -> &[OperationContent] {
        &self.contents
    }
}

impl From<SignedOperation> for UnsignedOperation {
    fn from(value: SignedOperation) -> Self {
        Self::new(value.branch, value.contents)
    }
}

/// The [SignedOperation] struct represent a signed tezos operation. It can be torned to an injectable string,
/// which can be injected into the Tezos blockchain using the `tezos-rpc` crate.
#[derive(Debug, Clone)]
pub struct SignedOperation {
    pub branch: BlockHash,
    pub contents: Vec<OperationContent>,
    pub signature: Signature,
}

impl SignedOperation {
    /// Verifies the operation signature with the given public key and using
    /// the crypto providers configured in the fiven [Tezos] instance.
    pub fn verify_with(&self, key: &PublicKey, tezos: &Tezos) -> Result<bool> {
        let signer = OperationSigner::new(tezos.get_crypto());
        signer.verify(self, key)
    }

    /// Verifies the operation signature with the given public key.
    pub fn verify(&self, key: &PublicKey) -> Result<bool> {
        let tezos: Tezos = Default::default();
        let signer = OperationSigner::new(tezos.get_crypto());
        signer.verify(self, key)
    }

    /// Returns an injectable string that can be used to inject the operation into the Tezos blockchain
    /// using the `tezos-rpc` crate.
    pub fn to_injectable_string(&self) -> Result<String> {
        let forged_bytes = self.to_forged_bytes()?;
        let signature_bytes = self.signature.to_bytes()?;
        Ok(hex::encode([forged_bytes, signature_bytes].concat()))
    }

    /// Creates a new instance of [SignedOperation].
    pub fn new(branch: BlockHash, contents: Vec<OperationContent>, signature: Signature) -> Self {
        Self {
            branch,
            contents,
            signature,
        }
    }

    /// Creates a new instance of [SignedOperation] from an [UnsignedOperation] and a [Signature].
    /// No validation of the signature is performed.
    pub fn from(operation: UnsignedOperation, signature: Signature) -> Self {
        Self::new(operation.branch, operation.contents, signature)
    }

    /// Static method that calculates and encodes operation hash given its forged bytes (including signature)
    pub fn operation_hash(payload: &[u8]) -> Result<OperationHash> {
        let crypto = Crypto::new(None, None, None);
        let hash = crypto.blake2b(payload, 32)?;
        OperationHash::from_bytes(&hash).map_err(|e| e.into())
    }

    /// Calculates operation hash
    pub fn hash(&self) -> Result<OperationHash> {
        let payload = [self.to_forged_bytes()?, self.signature.to_bytes()?].concat();
        Self::operation_hash(payload.as_slice())
    }
}

impl Operation for SignedOperation {
    fn branch(&self) -> &BlockHash {
        &self.branch
    }

    fn contents(&self) -> &[OperationContent] {
        &self.contents
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationContent {
    SeedNonceRevelation(SeedNonceRevelation),
    DoubleEndorsementEvidence(DoubleEndorsementEvidence),
    DoubleBakingEvidence(DoubleBakingEvidence),
    ActivateAccount(ActivateAccount),
    Proposals(Proposals),
    Ballot(Ballot),
    DoublePreendorsementEvidence(DoublePreendorsementEvidence),
    FailingNoop(FailingNoop),
    Preendorsement(Preendorsement),
    Endorsement(Endorsement),
    Reveal(Reveal),
    Transaction(Transaction),
    Origination(Origination),
    Delegation(Delegation),
    RegisterGlobalConstant(RegisterGlobalConstant),
    SetDepositsLimit(SetDepositsLimit),
}

impl OperationContent {
    pub fn to_forged_bytes(&self) -> Result<Vec<u8>> {
        OperationContentBytesCoder::encode(self)
    }

    pub fn from_forged_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        OperationContentBytesCoder::decode(bytes.as_ref())
    }

    pub fn fee(&self) -> Mutez {
        match self {
            Self::Reveal(value) => value.fee,
            Self::Transaction(value) => value.fee,
            Self::Origination(value) => value.fee,
            Self::Delegation(value) => value.fee,
            Self::RegisterGlobalConstant(value) => value.fee,
            Self::SetDepositsLimit(value) => value.fee,
            _ => 0u8.into(),
        }
    }

    pub fn has_fee(&self) -> bool {
        self.fee() != 0u8.into()
    }
}

impl From<SeedNonceRevelation> for OperationContent {
    fn from(value: SeedNonceRevelation) -> Self {
        Self::SeedNonceRevelation(value)
    }
}

impl From<DoubleEndorsementEvidence> for OperationContent {
    fn from(value: DoubleEndorsementEvidence) -> Self {
        Self::DoubleEndorsementEvidence(value)
    }
}

impl From<DoubleBakingEvidence> for OperationContent {
    fn from(value: DoubleBakingEvidence) -> Self {
        Self::DoubleBakingEvidence(value)
    }
}

impl From<ActivateAccount> for OperationContent {
    fn from(value: ActivateAccount) -> Self {
        Self::ActivateAccount(value)
    }
}

impl From<Proposals> for OperationContent {
    fn from(value: Proposals) -> Self {
        Self::Proposals(value)
    }
}

impl From<Ballot> for OperationContent {
    fn from(value: Ballot) -> Self {
        Self::Ballot(value)
    }
}

impl From<DoublePreendorsementEvidence> for OperationContent {
    fn from(value: DoublePreendorsementEvidence) -> Self {
        Self::DoublePreendorsementEvidence(value)
    }
}

impl From<FailingNoop> for OperationContent {
    fn from(value: FailingNoop) -> Self {
        Self::FailingNoop(value)
    }
}

impl From<Preendorsement> for OperationContent {
    fn from(value: Preendorsement) -> Self {
        Self::Preendorsement(value)
    }
}

impl From<Endorsement> for OperationContent {
    fn from(value: Endorsement) -> Self {
        Self::Endorsement(value)
    }
}

impl From<Reveal> for OperationContent {
    fn from(value: Reveal) -> Self {
        Self::Reveal(value)
    }
}

impl From<Transaction> for OperationContent {
    fn from(value: Transaction) -> Self {
        Self::Transaction(value)
    }
}

impl From<Origination> for OperationContent {
    fn from(value: Origination) -> Self {
        Self::Origination(value)
    }
}

impl From<Delegation> for OperationContent {
    fn from(value: Delegation) -> Self {
        Self::Delegation(value)
    }
}

impl From<RegisterGlobalConstant> for OperationContent {
    fn from(value: RegisterGlobalConstant) -> Self {
        Self::RegisterGlobalConstant(value)
    }
}

impl From<SetDepositsLimit> for OperationContent {
    fn from(value: SetDepositsLimit) -> Self {
        Self::SetDepositsLimit(value)
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum OperationContentTag {
    SeedNonceRevelation = 1,
    DoubleEndorsementEvidence = 2,
    DoubleBakingEvidence = 3,
    ActivateAccount = 4,
    Proposals = 5,
    Ballot = 6,
    DoublePreendorsementEvidence = 7,
    FailingNoop = 17,
    Preendorsement = 20,
    Endorsement = 21,
    Reveal = 107,
    Transaction = 108,
    Origination = 109,
    Delegation = 110,
    RegisterGlobalConstant = 111,
    SetDepositsLimit = 112,
}

impl OperationContentTag {
    pub fn to_bytes(&self) -> [u8; 1] {
        [*self as u8]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlinedEndorsement {
    pub branch: BlockHash,
    pub operations: Endorsement,
    pub signature: Signature,
}

impl InlinedEndorsement {
    pub fn new(branch: BlockHash, operations: Endorsement, signature: Signature) -> Self {
        Self {
            branch,
            operations,
            signature,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlinedPreendrosement {
    pub branch: BlockHash,
    pub operations: Preendorsement,
    pub signature: Signature,
}

impl InlinedPreendrosement {
    pub fn new(branch: BlockHash, operations: Preendorsement, signature: Signature) -> Self {
        Self {
            branch,
            operations,
            signature,
        }
    }
}

mod traits {
    use tezos_core::types::{
        encoded::{BlockPayloadHash, ImplicitAddress},
        mutez::Mutez,
        number::Nat,
    };

    use super::OperationContentTag;

    pub trait OperationContent {
        fn tag() -> OperationContentTag;
    }

    pub trait OperationConsensusContent {
        fn slot(&self) -> u16;
        fn level(&self) -> i32;
        fn round(&self) -> i32;
        fn block_payload_hash(&self) -> &BlockPayloadHash;
    }

    pub trait OperationManagerContent {
        fn source(&self) -> &ImplicitAddress;
        fn fee(&self) -> Mutez;
        fn counter(&self) -> &Nat;
        fn gas_limit(&self) -> &Nat;
        fn storage_limit(&self) -> &Nat;
    }
}

#[cfg(test)]
mod test {
    use crate::block_header::{BlockHeader, LiquidityBakingToggleVote};

    use super::*;
    use chrono::DateTime;
    use hex_literal::hex;

    #[test]
    fn test_forge_operation() -> Result<()> {
        for (operation, bytes) in operations_with_bytes() {
            let forged = operation.to_forged_bytes()?;
            assert_eq!(bytes, forged);
        }
        Ok(())
    }

    #[test]
    fn test_unforge_operation() -> Result<()> {
        for (operation, bytes) in operations_with_bytes() {
            let unforged = UnsignedOperation::from_forged_bytes(bytes)?;
            assert_eq!(operation, unforged);
        }
        Ok(())
    }

    #[test]
    fn test_forge_operation_content() -> Result<()> {
        for (operation, bytes) in operation_contents_with_bytes() {
            let forged = operation.to_forged_bytes()?;
            assert_eq!(bytes, forged);
        }
        Ok(())
    }

    #[test]
    fn test_unforge_operation_content() -> Result<()> {
        for (operation, bytes) in operation_contents_with_bytes() {
            let unforged_operation = OperationContent::from_forged_bytes(bytes)?;
            assert_eq!(operation, unforged_operation);
        }
        Ok(())
    }

    fn operations_with_bytes() -> Vec<(UnsignedOperation, &'static [u8])> {
        vec![
            (
                UnsignedOperation::new("BLyKu3tnc9NCuiFfCqfeVGPCoZTyW63dYh2XAYxkM7fQYKCqsju".try_into().unwrap(), vec![]),
                &hex!("a5db12a8a7716fa5445bd374c8b3239c876dde8397efae0eb0dd223dc23a51c7")
            ),
            (
                UnsignedOperation::new(
                    "BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(),
                    vec![SeedNonceRevelation::new(1, "6cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214".try_into().unwrap()).into()]
                ),
                &hex!("86db32fcecf30277eef3ef9f397118ed067957dd998979fd723ea0a0d50beead01000000016cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214")
            ),
            (
                UnsignedOperation::new(
                    "BLjg4HU2BwnCgJfRutxJX5rHACzLDxRJes1MXqbXXdxvHWdK3Te".try_into().unwrap(),
                    vec![
                        SeedNonceRevelation::new(1, "9d15bcdc0194b327d3cb0dcd05242bc6ff1635da635e38ed7a62b8c413ce6833".try_into().unwrap()).into(),
                        SeedNonceRevelation::new(2, "921ed0115c7cc1b5dcd07ad66ce4d9b2b0186c93c27a80d70b66b4e309add170".try_into().unwrap()).into(),
                    ]
                ),
                &hex!("86db32fcecf30277eef3ef9f397118ed067957dd998979fd723ea0a0d50beead01000000019d15bcdc0194b327d3cb0dcd05242bc6ff1635da635e38ed7a62b8c413ce68330100000002921ed0115c7cc1b5dcd07ad66ce4d9b2b0186c93c27a80d70b66b4e309add170")
            ),
        ]
    }

    fn operation_contents_with_bytes() -> Vec<(OperationContent, &'static [u8])> {
        vec![
            (
                SeedNonceRevelation::new(
                    1,
                    "6cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214"
                        .try_into()
                        .unwrap(),
                )
                .into(),
                &hex!("01000000016cdaf9367e551995a670a5c642a9396290f8c9d17e6bc3c1555bfaa910d92214"),
            ),
            (
                DoubleEndorsementEvidence::new(
                    InlinedEndorsement::new(
                        "BLT3XKN3vFqWnWfuuLenQiyVgEgKcJttnGGdCcQbmE95xz9y7S5".try_into().unwrap(),
                        Endorsement::new(
                            1,
                            1,
                            1,
                            "vh2cHpyeaHQhF7g3RFh8usyYmTTpt882UsRyXECuBwPiB3TcsKNd".try_into().unwrap(),
                        ),
                        "sigdV5DNZRBLBDDEkbWcqefBuMZevanVyjotoazkkLbk7jXR8oZUmnxt6n3hkQtTe9WbLEkcCUWw1Ey7Ybby5z35nHKqpndn".try_into().unwrap(),
                    ),
                    InlinedEndorsement::new(
                        "BLZS5mP4BufHrZfvzrvw1ReWnj1L2zcQ4mM6Jywoaxe4mHbiCNn".try_into().unwrap(),
                        Endorsement::new(
                            2,
                            2,
                            2,
                            "vh2rXj5TAG8p1HKiMyaWDdYrRL2rTBPyFLkVorgzEEBqqd4sgsXG".try_into().unwrap(),
                        ),
                        "sigff9imsFxGwyQ8nEpXUR8ZFwTqZWjMJAgKGwub6Mn9Cnu4VvBppTRt84VPp1fRwqpx8JTrLHg76guTGzkm9ETKwFNCzniY".try_into().unwrap(),
                    ),
                )
                .into(),
                &hex!("020000008b611895c74249d0a90db97644942543d9a9f9efdf48f6fae039f1f72b07ad9ed415000100000001000000017afe70591b8fce15d79383d3b2d1215e11d49672901d733842d6221562a98324767251a73e10b6bbe72a662576abb35bb3161f9a662ead7207e26ca95dbd1c0a3b086470822e83160f916415e00f07840cecfb897e61945255c3ab943bebc1e60000008b6f9a5a686491dc1af62fe3f0c3b2d8d6e1f5883f50592029980d55864a6b24b015000200000002000000029b53a37d056c73de29fef1e17abfaab06876147aa7083b52b0ef6ba92bf5a50c870fd592cf831578551c230a5cc324c7d26c67e5185f071b3fdb797ef89f3be013d51b0f3cf181cb842f13bf35c29a2343908b348b7b5db2e38caa505d5dfc34")
            ),
            (
                DoubleBakingEvidence::new(
                    BlockHeader {
                        level: 1,
                        proto: 1,
                        predecessor: "BKsP8FYgikDmqbUiVxfgXVjWuay5LQZY6LP4EvcsFK8uuqj4wQD".try_into().unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.001Z").unwrap().naive_utc(),
                        validation_pass: 1,
                        operations_hash: "LLoaLP6mc6nVzG2Rp3fSrHFvvGpUvkbHCjLASVduN7GzQAKnPctrR".try_into().unwrap(),
                        fitness: vec![],
                        context: "CoWKSZnE72uMLBeh3Fmj3LSXjJmeCEmYBMxAig15g3LPjTP4rHmR".try_into().unwrap(),
                        payload_hash: "vh2cJrNF6FCXo1bfnM9hj66NDQSGQCBxTtqkxkMLzkTeeDnZjrvD".try_into().unwrap(),
                        payload_round: 1,
                        proof_of_work_nonce: "d4d34b5686c98ae1".try_into().unwrap(),
                        seed_nonce_hash: None,
                        liquidity_baking_toggle_vote: LiquidityBakingToggleVote::On,
                        signature: "sigiaEd9dHEGKgccx3JBBDw4eb6WVxGH3MvyziYbQqWQRMmyecdo5VuSkYWkgZvcQXshB4vV2qkTb6AxbKruaNPfnMg4u2EA".try_into().unwrap()
                    },
                    BlockHeader {
                        level: 2,
                        proto: 2,
                        predecessor: "BMaBxGyVhtTiMKd7KA8HXJnbTK4e1TzffNc94G18op55HGQYVRk".try_into().unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.002Z").unwrap().naive_utc(),
                        validation_pass: 2,
                        operations_hash: "LLoaNF9sd5z2SZtSmUopYNX6qs77QAUJqrnd5ei378H4bcJhQcPt5".try_into().unwrap(),
                        fitness: vec![],
                        context: "CoVj5HxwnPHpC1SgCC6pgqVPgw2vqFEqaC2bF5STqcbyX6giVrGn".try_into().unwrap(),
                        payload_hash: "vh2MHqgJtw8v7CDrZKYWtLmqGJtjzkRvs9yUeHNQqdgDJyCYm21q".try_into().unwrap(),
                        payload_round: 2,
                        proof_of_work_nonce: "336ebf95efce0475".try_into().unwrap(),
                        seed_nonce_hash: Some("nceUeUCJRZ4M7FCSBsAUZU6dmxePdH7irje9Gfj9zWwCdfWd5B4Ee".try_into().unwrap()),
                        liquidity_baking_toggle_vote: LiquidityBakingToggleVote::Off,
                        signature: "sigRsUhHqaFVBeV4qzyCZ6Y9TvoKajyNwyPQQCW3SbgPYY99MrpTqR2FopjzZEHMWoJG7LaTaHu7bnieKQRKqCRLA7hB7Ekp".try_into().unwrap()
                    },
                )
                .into(),
                &hex!("03000000e0000000010114a98b361825acd1997319b0b01069908d1103df26a5646bf998cd6df80b95c60000000000000001018539ef2bf06ca139c6aeda9edc16c853f2b09ff232fab97d7a15150a602ea36500000000dc8d5cafd036ba185119ba904aefbdefd6d30de1f5e4a49fb20b0997ea2cdc357b08b37679350e62ea1bff3287d151c79156f0160b296bdade0ffa7f16f26b6300000001d4d34b5686c98ae100009d584824e3bf8b4817abdce782d94d93df6c60581e581990767cb8c0c07c577c328cddebd2da2433736411e17c2cfb282c8067e89c5a3e48246f50eca5e7525f000001000000000202f5043ad9d3aeea868db43f2abda52e1b7f176f928742964ce1db62d8f48cd67f0000000000000002028974da4dc7fcb31faab671f35d065db1d699a2b7d97bb830330977b8650591b0000000008e84ab5712175f8ab1ce14bcf5185d712c472a4e6abf51093a06c7e9042e59d258ef5ec7e36bb4004a4e7f10cb94032d59b65f8a86450c20a63d802ad749546200000002336ebf95efce0475ff37ad10c119adb450d7456104f3971536fb486124a262549c00d3310cd93e6820011dad11dad4d16f110476a24734b1414725506b354e01de4e54a4fdcec01604fda840b53f2cac4109c32680fe58600d96749b1d2891a0aa22b222ba36c864f001")
            ),
            (
                DoubleBakingEvidence::new(
                    BlockHeader {
                        level: 1,
                        proto: 1,
                        predecessor: "BKsP8FYgikDmqbUiVxfgXVjWuay5LQZY6LP4EvcsFK8uuqj4wQD".try_into().unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.001Z").unwrap().naive_utc(),
                        validation_pass: 1,
                        operations_hash: "LLoaLP6mc6nVzG2Rp3fSrHFvvGpUvkbHCjLASVduN7GzQAKnPctrR".try_into().unwrap(),
                        fitness: vec!["00000001000000000100000001".try_into().unwrap()],
                        context: "CoWKSZnE72uMLBeh3Fmj3LSXjJmeCEmYBMxAig15g3LPjTP4rHmR".try_into().unwrap(),
                        payload_hash: "vh2cJrNF6FCXo1bfnM9hj66NDQSGQCBxTtqkxkMLzkTeeDnZjrvD".try_into().unwrap(),
                        payload_round: 1,
                        proof_of_work_nonce: "d4d34b5686c98ae1".try_into().unwrap(),
                        seed_nonce_hash: None,
                        liquidity_baking_toggle_vote: LiquidityBakingToggleVote::Off,
                        signature: "sigiaEd9dHEGKgccx3JBBDw4eb6WVxGH3MvyziYbQqWQRMmyecdo5VuSkYWkgZvcQXshB4vV2qkTb6AxbKruaNPfnMg4u2EA".try_into().unwrap()
                    },
                    BlockHeader {
                        level: 2,
                        proto: 2,
                        predecessor: "BMaBxGyVhtTiMKd7KA8HXJnbTK4e1TzffNc94G18op55HGQYVRk".try_into().unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.002Z").unwrap().naive_utc(),
                        validation_pass: 2,
                        operations_hash: "LLoaNF9sd5z2SZtSmUopYNX6qs77QAUJqrnd5ei378H4bcJhQcPt5".try_into().unwrap(),
                        fitness: vec!["00000002ff000000020000000200000002".try_into().unwrap(), "00000002000000000200000002".try_into().unwrap()],
                        context: "CoVj5HxwnPHpC1SgCC6pgqVPgw2vqFEqaC2bF5STqcbyX6giVrGn".try_into().unwrap(),
                        payload_hash: "vh2MHqgJtw8v7CDrZKYWtLmqGJtjzkRvs9yUeHNQqdgDJyCYm21q".try_into().unwrap(),
                        payload_round: 2,
                        proof_of_work_nonce: "336ebf95efce0475".try_into().unwrap(),
                        seed_nonce_hash: Some("nceUeUCJRZ4M7FCSBsAUZU6dmxePdH7irje9Gfj9zWwCdfWd5B4Ee".try_into().unwrap()),
                        liquidity_baking_toggle_vote: LiquidityBakingToggleVote::Pass,
                        signature: "sigRsUhHqaFVBeV4qzyCZ6Y9TvoKajyNwyPQQCW3SbgPYY99MrpTqR2FopjzZEHMWoJG7LaTaHu7bnieKQRKqCRLA7hB7Ekp".try_into().unwrap()
                    },
                )
                .into(),
                &hex!("03000000f1000000010114a98b361825acd1997319b0b01069908d1103df26a5646bf998cd6df80b95c60000000000000001018539ef2bf06ca139c6aeda9edc16c853f2b09ff232fab97d7a15150a602ea365000000110000000d00000001000000000100000001dc8d5cafd036ba185119ba904aefbdefd6d30de1f5e4a49fb20b0997ea2cdc357b08b37679350e62ea1bff3287d151c79156f0160b296bdade0ffa7f16f26b6300000001d4d34b5686c98ae100019d584824e3bf8b4817abdce782d94d93df6c60581e581990767cb8c0c07c577c328cddebd2da2433736411e17c2cfb282c8067e89c5a3e48246f50eca5e7525f000001260000000202f5043ad9d3aeea868db43f2abda52e1b7f176f928742964ce1db62d8f48cd67f0000000000000002028974da4dc7fcb31faab671f35d065db1d699a2b7d97bb830330977b8650591b0000000260000001100000002ff0000000200000002000000020000000d000000020000000002000000028e84ab5712175f8ab1ce14bcf5185d712c472a4e6abf51093a06c7e9042e59d258ef5ec7e36bb4004a4e7f10cb94032d59b65f8a86450c20a63d802ad749546200000002336ebf95efce0475ff37ad10c119adb450d7456104f3971536fb486124a262549c00d3310cd93e6820021dad11dad4d16f110476a24734b1414725506b354e01de4e54a4fdcec01604fda840b53f2cac4109c32680fe58600d96749b1d2891a0aa22b222ba36c864f001")
            ),
            (
                ActivateAccount::new("tz1PokEhtiBGCmekQrcN87pCDmqy99TjaLuN".try_into().unwrap(), "7b27ba02550e6834b50173c8c506de42d901c606".try_into().unwrap()).into(),
                &hex!("042db6ed2d71e8f22ce348c1b7b2e7f08892bd50ef7b27ba02550e6834b50173c8c506de42d901c606"),
            ),
            (
                Proposals::new("tz1QVzD6eV73LhtzhNKs94fKbvTg7VjKjEcE".try_into().unwrap(), 1, vec!["PtYnGfhwjiRjtA7VZriogYL6nwFgaAL9ZuVWE6UahXCMn6BoJPv".try_into().unwrap()]).into(),
                &hex!("050035533a79b20d6ea4dc8b92ab1cf33b448b93c78f0000000100000020f0e14a6c55f809a0ac08dc9bba0596b0daac1944520dfa9b8e2ce4e1a102a203"),
            ),
            (
                Proposals::new("tz1MRrtJC9sk1o1D57LPWev6DDVjMgra5pXb".try_into().unwrap(), 1, vec!["PtARwRL7jEGtzoCCWDBXe6ZJ4ZJiWtDgBC2a5WwnHYYyKPdmwrb".try_into().unwrap(), "Ps6NdX1CpeF3kHV5CVQZMLZKZwAN8NYN9crdL3GzEg4uNg7f3DY".try_into().unwrap()]).into(),
                &hex!("050013a312c56ed0eb53799ce6ef3eabfc1102f73b940000000100000040be2160f0cad3ca52a8e1a2f9e6fb25e748a769267dad2964550e6d946d0a03c23138e6c4f4e5e47064cbd0cd36b2a09ad01d0709b9737b1f8622a43448de01d5"),
            ),
            (
                Ballot::new("tz1eNhmMTYsti2quW46a5CBJbs4Fde4KGg4F".try_into().unwrap(), 1, "PsjL76mH8vo3fTfUN4qKrdkPvRfXw7KJPWf87isNAxzh1vqdFQv".try_into().unwrap(), BallotType::Yay).into(),
                &hex!("0600cd8459db8668d3ae6a4f49cb8fe3c5bbd6c76956000000018522ef9f87cef2f745984cdbfe4a723acfbe7979c6f24ebc04a86d786b1c038500"),
            ),
            (
                Ballot::new("tz1eNhmMTYsti2quW46a5CBJbs4Fde4KGg4F".try_into().unwrap(), 1, "PsjL76mH8vo3fTfUN4qKrdkPvRfXw7KJPWf87isNAxzh1vqdFQv".try_into().unwrap(), BallotType::Nay).into(),
                &hex!("0600cd8459db8668d3ae6a4f49cb8fe3c5bbd6c76956000000018522ef9f87cef2f745984cdbfe4a723acfbe7979c6f24ebc04a86d786b1c038501"),
            ),
            (
                Ballot::new("tz1eNhmMTYsti2quW46a5CBJbs4Fde4KGg4F".try_into().unwrap(), 1, "PsjL76mH8vo3fTfUN4qKrdkPvRfXw7KJPWf87isNAxzh1vqdFQv".try_into().unwrap(), BallotType::Pass).into(),
                &hex!("0600cd8459db8668d3ae6a4f49cb8fe3c5bbd6c76956000000018522ef9f87cef2f745984cdbfe4a723acfbe7979c6f24ebc04a86d786b1c038502"),
            ),
            (
                DoublePreendorsementEvidence::new(
                    InlinedPreendrosement {
                        branch: "BLT3XKN3vFqWnWfuuLenQiyVgEgKcJttnGGdCcQbmE95xz9y7S5".try_into().unwrap(),
                        operations: Preendorsement::new(1, 1, 1, "vh2cHpyeaHQhF7g3RFh8usyYmTTpt882UsRyXECuBwPiB3TcsKNd".try_into().unwrap()),
                        signature: "sigdV5DNZRBLBDDEkbWcqefBuMZevanVyjotoazkkLbk7jXR8oZUmnxt6n3hkQtTe9WbLEkcCUWw1Ey7Ybby5z35nHKqpndn".try_into().unwrap(),
                    },
                    InlinedPreendrosement {
                        branch: "BLZS5mP4BufHrZfvzrvw1ReWnj1L2zcQ4mM6Jywoaxe4mHbiCNn".try_into().unwrap(),
                        operations: Preendorsement::new(2, 2, 2, "vh2rXj5TAG8p1HKiMyaWDdYrRL2rTBPyFLkVorgzEEBqqd4sgsXG".try_into().unwrap()),
                        signature: "sigff9imsFxGwyQ8nEpXUR8ZFwTqZWjMJAgKGwub6Mn9Cnu4VvBppTRt84VPp1fRwqpx8JTrLHg76guTGzkm9ETKwFNCzniY".try_into().unwrap(),
                    },
                ).into(),
                &hex!("070000008b611895c74249d0a90db97644942543d9a9f9efdf48f6fae039f1f72b07ad9ed414000100000001000000017afe70591b8fce15d79383d3b2d1215e11d49672901d733842d6221562a98324767251a73e10b6bbe72a662576abb35bb3161f9a662ead7207e26ca95dbd1c0a3b086470822e83160f916415e00f07840cecfb897e61945255c3ab943bebc1e60000008b6f9a5a686491dc1af62fe3f0c3b2d8d6e1f5883f50592029980d55864a6b24b014000200000002000000029b53a37d056c73de29fef1e17abfaab06876147aa7083b52b0ef6ba92bf5a50c870fd592cf831578551c230a5cc324c7d26c67e5185f071b3fdb797ef89f3be013d51b0f3cf181cb842f13bf35c29a2343908b348b7b5db2e38caa505d5dfc34"),
            ),
            (
                FailingNoop::new("cc7e647be422e432a3291ec8a2ee6f5e2210c51825b753758a99e266a0c65b15".try_into().unwrap()).into(),
                &hex!("1100000020cc7e647be422e432a3291ec8a2ee6f5e2210c51825b753758a99e266a0c65b15"),
            ),
            (
                Preendorsement::new(1, 1, 1, "vh2KDvhtt44Lyq187SnZjSDyRH1LNXbMj3T9G57miWK9QvqH3fhv".try_into().unwrap()).into(),
                &hex!("1400010000000100000001543d9791df12f3237de836314a45a348e5d608c80a6a411246dfc67ef1a08d0a"),
            ),
            (
                Endorsement::new(1, 1, 1, "vh2WtVuY9PK3mDsnfdzA6iXc4pocgUff8hgamWwXw19r5kDYHVS5".try_into().unwrap()).into(),
                &hex!("15000100000001000000016eba3d57f131a71eab0692e333e889cbafe523c675e588ace92bb5056cbcb889"),
            ),
            (
                Reveal::new("tz1SZ2CmbQB7MMXgcMSmyyVXpya1rkb9UGUE".try_into().unwrap(), "135675".try_into().unwrap(), "154".try_into().unwrap(), "23675".try_into().unwrap(), "34152".try_into().unwrap(), "edpkuaARNJPQygG82X1xed6Z2kDutT8XjL3Fmv1XPBbca1uARirj55".try_into().unwrap()).into(),
                &hex!("6b004bd66485632a18d61068fc940772dec8add5ff93fba3089a01fbb801e88a02007a79d89acb296dd9ec2be8fba817702dc41adf19e28bb250a337f840eb263c69"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "KT1GFYUFQRT4RsNbtG2NU23woUyMp5tx9gx2".try_into().unwrap(), None).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e01541e2bf7dc4401328be301227d204d5dc233b6760000"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "tz1YbTdYqmpLatAqLb1sm67qqXMXyRB3UYiz".try_into().unwrap(), Some(Parameters::new(Entrypoint::default(), vec![].into()))).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e00008e1d34730fcd7e8282b0efe7b09b3c57543e59c8ff00000000050200000000"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "tz1YbTdYqmpLatAqLb1sm67qqXMXyRB3UYiz".try_into().unwrap(), Some(Parameters::new(Entrypoint::Root, vec![].into()))).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e00008e1d34730fcd7e8282b0efe7b09b3c57543e59c8ff01000000050200000000"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "tz1YbTdYqmpLatAqLb1sm67qqXMXyRB3UYiz".try_into().unwrap(), Some(Parameters::new(Entrypoint::Do, vec![].into()))).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e00008e1d34730fcd7e8282b0efe7b09b3c57543e59c8ff02000000050200000000"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "tz1YbTdYqmpLatAqLb1sm67qqXMXyRB3UYiz".try_into().unwrap(), Some(Parameters::new(Entrypoint::SetDelegate, vec![].into()))).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e00008e1d34730fcd7e8282b0efe7b09b3c57543e59c8ff03000000050200000000"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "tz1YbTdYqmpLatAqLb1sm67qqXMXyRB3UYiz".try_into().unwrap(), Some(Parameters::new(Entrypoint::RemoveDelegate, vec![].into()))).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e00008e1d34730fcd7e8282b0efe7b09b3c57543e59c8ff04000000050200000000"),
            ),
            (
                Transaction::new("tz1i8xLzLPQHknc5jmeFc3qxijar2HLG2W4Z".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), "tz1YbTdYqmpLatAqLb1sm67qqXMXyRB3UYiz".try_into().unwrap(), Some(Parameters::new(Entrypoint::Named("named".into()), vec![].into()))).into(),
                &hex!("6c00f6cb338e136f281d17a2657437f090daf84b42affba3089a01fbb801e88a02ebca2e00008e1d34730fcd7e8282b0efe7b09b3c57543e59c8ffff056e616d6564000000050200000000"),
            ),
            (
                Origination::new("tz1LdF7qHCJg8Efa6Cx4LZrRPkvbh61H8tZq".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), None, Script::new(vec![].into(), vec![].into())).into(),
                &hex!("6d000ad2152600ac6bb16b5512e43a337dd562dc2cccfba3089a01fbb801e88a02ebca2e00000000050200000000000000050200000000"),
            ),
            (
                Origination::new("tz1LdF7qHCJg8Efa6Cx4LZrRPkvbh61H8tZq".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), 763243u32.into(), Some("tz1RY8er4ybXszZBbhtQDrYhA5AYY3VQXiKn".try_into().unwrap()), Script::new(vec![].into(), vec![].into())).into(),
                &hex!("6d000ad2152600ac6bb16b5512e43a337dd562dc2cccfba3089a01fbb801e88a02ebca2eff0040b33c1a35d72f3c85747f605b1902d36fc8c9a3000000050200000000000000050200000000"),
            ),
            (
                Delegation::new("tz1QVAraV1JDRsPikcqJVE4VccvW7vDWCJHy".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), None).into(),
                &hex!("6e00352bb30ffdb72d101083a4fc5cd156f007705f5dfba3089a01fbb801e88a0200"),
            ),
            (
                Delegation::new("tz1QVAraV1JDRsPikcqJVE4VccvW7vDWCJHy".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), Some("tz1dStZpfk5bWsvYvuktDJgDEbpuqDc7ipvi".try_into().unwrap())).into(),
                &hex!("6e00352bb30ffdb72d101083a4fc5cd156f007705f5dfba3089a01fbb801e88a02ff00c356e7cb9943f6ef4168bea7915c7f88152e6c37"),
            ),
            (
                RegisterGlobalConstant::new("tz1brHnNaHcpxqHDhqwmAXDq1i4F2A4Xaepz".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), vec![].into()).into(),
                &hex!("6f00b1d399df432bbbdbd45cb6b454699ea96d77dabffba3089a01fbb801e88a02000000050200000000"),
            ),
            (
                SetDepositsLimit::new("tz1gxabEuUaCKk15qUKnhASJJoXhm9A7DVLM".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), None).into(),
                &hex!("7000e9dcc1a4a82c49aeec327b15e9ed457dc22a1ebcfba3089a01fbb801e88a0200"),
            ),
            (
                SetDepositsLimit::new("tz1gxabEuUaCKk15qUKnhASJJoXhm9A7DVLM".try_into().unwrap(), 135675u32.into(), 154u32.into(), 23675u32.into(), 34152u32.into(), Some(634u32.into())).into(),
                &hex!("7000e9dcc1a4a82c49aeec327b15e9ed457dc22a1ebcfba3089a01fbb801e88a02fffa04"),
            ),
        ]
    }

    #[test]
    fn test_operation_hash() -> Result<()> {
        let opg = SignedOperation::new(
            "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC".try_into().unwrap(),
            vec![
                Transaction::new(
                    "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU".try_into().unwrap(),
                    417u32.into(),
                    2336132u32.into(),
                    1527u32.into(),
                    357u32.into(),
                    498719u32.into(),
                    "tz1d5Dr3gjsxQo5XNbjAj558mLy3nGGQgMFA".try_into().unwrap(),
                    None
                ).into()
            ],
            "sigw1WNdYweqz1c7zKcvZFHQ18swSv4HBWje5quRmixxitPk7z8jtY63qXgKLPVfTM6XGxExPatBWJP44Bknyu3hDHDKJZgY".try_into().unwrap()
        );
        let expected: OperationHash = "onpLA98fWzC1xEhfJ19PvvAn4NireAKGvzXZbdCgQyRvbxRjFuD"
            .try_into()
            .unwrap();
        let actual = opg.hash()?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
