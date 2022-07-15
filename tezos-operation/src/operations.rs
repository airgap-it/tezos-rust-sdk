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
    internal::coder::{Decoder, Encoder},
    types::encoded::{BlockHash, Signature},
};

use crate::{
    internal::coder::{
        operation_bytes_coder::OperationBytesCoder,
        operation_content_bytes_coder::OperationContentBytesCoder,
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
    transaction::{CommonEntrypoint, Entrypoint, Parameters, Transaction},
};

pub trait Operation {
    fn branch(&self) -> &BlockHash;
    fn contents(&self) -> &[OperationContent];

    fn to_forged_bytes(&self) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        OperationBytesCoder::encode(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedOperation {
    branch: BlockHash,
    contents: Vec<OperationContent>,
}

impl UnsignedOperation {
    pub fn new(branch: BlockHash, contents: Vec<OperationContent>) -> Self {
        Self { branch, contents }
    }

    pub fn from_forged_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        OperationBytesCoder::decode(bytes.as_ref())
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

#[derive(Debug, Clone)]
pub struct SignedOperation {
    branch: BlockHash,
    contents: Vec<OperationContent>,
    signature: Signature,
}

impl SignedOperation {
    pub fn signature(&self) -> &Signature {
        &self.signature
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
    branch: BlockHash,
    operations: Endorsement,
    signature: Signature,
}

impl InlinedEndorsement {
    pub fn branch(&self) -> &BlockHash {
        &self.branch
    }

    pub fn operations(&self) -> &Endorsement {
        &self.operations
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

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
    branch: BlockHash,
    operations: Preendorsement,
    signature: Signature,
}

impl InlinedPreendrosement {
    pub fn branch(&self) -> &BlockHash {
        &self.branch
    }

    pub fn operations(&self) -> &Preendorsement {
        &self.operations
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

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
    use crate::block_header::BlockHeader;

    use super::*;
    use chrono::DateTime;
    use hex_literal::hex;
    use tezos_core::types::encoded::{
        BlockHash, BlockPayloadHash, ContextHash, Encoded, GenericSignature, NonceHash,
        OperationListListHash,
    };

    #[test]
    fn test_forge() -> Result<()> {
        for (operation, bytes) in operations_with_bytes() {
            let forged = operation.to_forged_bytes()?;
            assert_eq!(hex::encode(bytes), hex::encode(forged));
        }
        Ok(())
    }

    #[test]
    fn test_unforge() -> Result<()> {
        for (operation, bytes) in operations_with_bytes() {
            let unforged_operation = OperationContent::from_forged_bytes(bytes)?;
            assert_eq!(operation, unforged_operation);
        }
        Ok(())
    }

    fn operations_with_bytes() -> Vec<(OperationContent, &'static [u8])> {
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
                        BlockHash::new("BLT3XKN3vFqWnWfuuLenQiyVgEgKcJttnGGdCcQbmE95xz9y7S5".into()).unwrap(),
                        Endorsement::new(
                            1,
                            1,
                            1,
                            BlockPayloadHash::new("vh2cHpyeaHQhF7g3RFh8usyYmTTpt882UsRyXECuBwPiB3TcsKNd".into()).unwrap(),
                        ),
                        GenericSignature::new("sigdV5DNZRBLBDDEkbWcqefBuMZevanVyjotoazkkLbk7jXR8oZUmnxt6n3hkQtTe9WbLEkcCUWw1Ey7Ybby5z35nHKqpndn".into()).unwrap().into(),
                    ),
                    InlinedEndorsement::new(
                        BlockHash::new("BLZS5mP4BufHrZfvzrvw1ReWnj1L2zcQ4mM6Jywoaxe4mHbiCNn".into()).unwrap(),
                        Endorsement::new(
                            2,
                            2,
                            2,
                            BlockPayloadHash::new("vh2rXj5TAG8p1HKiMyaWDdYrRL2rTBPyFLkVorgzEEBqqd4sgsXG".into()).unwrap(),
                        ),
                        GenericSignature::new("sigff9imsFxGwyQ8nEpXUR8ZFwTqZWjMJAgKGwub6Mn9Cnu4VvBppTRt84VPp1fRwqpx8JTrLHg76guTGzkm9ETKwFNCzniY".into()).unwrap().into(),
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
                        predecessor: BlockHash::new("BKsP8FYgikDmqbUiVxfgXVjWuay5LQZY6LP4EvcsFK8uuqj4wQD".into()).unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.001Z").unwrap().naive_utc(),
                        validation_pass: 1,
                        operations_hash: OperationListListHash::new("LLoaLP6mc6nVzG2Rp3fSrHFvvGpUvkbHCjLASVduN7GzQAKnPctrR".into()).unwrap(),
                        fitness: vec![],
                        context: ContextHash::new("CoWKSZnE72uMLBeh3Fmj3LSXjJmeCEmYBMxAig15g3LPjTP4rHmR".into()).unwrap(),
                        payload_hash: BlockPayloadHash::new("vh2cJrNF6FCXo1bfnM9hj66NDQSGQCBxTtqkxkMLzkTeeDnZjrvD".into()).unwrap(),
                        payload_round: 1,
                        proof_of_work_nonce: "d4d34b5686c98ae1".try_into().unwrap(),
                        seed_nonce_hash: None,
                        liquidity_baking_escape_vote: true,
                        signature: GenericSignature::new("sigiaEd9dHEGKgccx3JBBDw4eb6WVxGH3MvyziYbQqWQRMmyecdo5VuSkYWkgZvcQXshB4vV2qkTb6AxbKruaNPfnMg4u2EA".into()).unwrap().into()
                    },
                    BlockHeader {
                        level: 2,
                        proto: 2,
                        predecessor: BlockHash::new("BMaBxGyVhtTiMKd7KA8HXJnbTK4e1TzffNc94G18op55HGQYVRk".into()).unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.002Z").unwrap().naive_utc(),
                        validation_pass: 2,
                        operations_hash: OperationListListHash::new("LLoaNF9sd5z2SZtSmUopYNX6qs77QAUJqrnd5ei378H4bcJhQcPt5".into()).unwrap(),
                        fitness: vec![],
                        context: ContextHash::new("CoVj5HxwnPHpC1SgCC6pgqVPgw2vqFEqaC2bF5STqcbyX6giVrGn".into()).unwrap(),
                        payload_hash: BlockPayloadHash::new("vh2MHqgJtw8v7CDrZKYWtLmqGJtjzkRvs9yUeHNQqdgDJyCYm21q".into()).unwrap(),
                        payload_round: 2,
                        proof_of_work_nonce: "336ebf95efce0475".try_into().unwrap(),
                        seed_nonce_hash: Some(NonceHash::new("nceUeUCJRZ4M7FCSBsAUZU6dmxePdH7irje9Gfj9zWwCdfWd5B4Ee".into()).unwrap()),
                        liquidity_baking_escape_vote: false,
                        signature: GenericSignature::new("sigRsUhHqaFVBeV4qzyCZ6Y9TvoKajyNwyPQQCW3SbgPYY99MrpTqR2FopjzZEHMWoJG7LaTaHu7bnieKQRKqCRLA7hB7Ekp".into()).unwrap().into()
                    },
                )
                .into(),
                &hex!("03000000e0000000010114a98b361825acd1997319b0b01069908d1103df26a5646bf998cd6df80b95c60000000000000001018539ef2bf06ca139c6aeda9edc16c853f2b09ff232fab97d7a15150a602ea36500000000dc8d5cafd036ba185119ba904aefbdefd6d30de1f5e4a49fb20b0997ea2cdc357b08b37679350e62ea1bff3287d151c79156f0160b296bdade0ffa7f16f26b6300000001d4d34b5686c98ae100ff9d584824e3bf8b4817abdce782d94d93df6c60581e581990767cb8c0c07c577c328cddebd2da2433736411e17c2cfb282c8067e89c5a3e48246f50eca5e7525f000001000000000202f5043ad9d3aeea868db43f2abda52e1b7f176f928742964ce1db62d8f48cd67f0000000000000002028974da4dc7fcb31faab671f35d065db1d699a2b7d97bb830330977b8650591b0000000008e84ab5712175f8ab1ce14bcf5185d712c472a4e6abf51093a06c7e9042e59d258ef5ec7e36bb4004a4e7f10cb94032d59b65f8a86450c20a63d802ad749546200000002336ebf95efce0475ff37ad10c119adb450d7456104f3971536fb486124a262549c00d3310cd93e6820001dad11dad4d16f110476a24734b1414725506b354e01de4e54a4fdcec01604fda840b53f2cac4109c32680fe58600d96749b1d2891a0aa22b222ba36c864f001")
            ),
            (
                DoubleBakingEvidence::new(
                    BlockHeader {
                        level: 1,
                        proto: 1,
                        predecessor: BlockHash::new("BKsP8FYgikDmqbUiVxfgXVjWuay5LQZY6LP4EvcsFK8uuqj4wQD".into()).unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.001Z").unwrap().naive_utc(),
                        validation_pass: 1,
                        operations_hash: OperationListListHash::new("LLoaLP6mc6nVzG2Rp3fSrHFvvGpUvkbHCjLASVduN7GzQAKnPctrR".into()).unwrap(),
                        fitness: vec!["00000001000000000100000001".try_into().unwrap()],
                        context: ContextHash::new("CoWKSZnE72uMLBeh3Fmj3LSXjJmeCEmYBMxAig15g3LPjTP4rHmR".into()).unwrap(),
                        payload_hash: BlockPayloadHash::new("vh2cJrNF6FCXo1bfnM9hj66NDQSGQCBxTtqkxkMLzkTeeDnZjrvD".into()).unwrap(),
                        payload_round: 1,
                        proof_of_work_nonce: "d4d34b5686c98ae1".try_into().unwrap(),
                        seed_nonce_hash: None,
                        liquidity_baking_escape_vote: true,
                        signature: GenericSignature::new("sigiaEd9dHEGKgccx3JBBDw4eb6WVxGH3MvyziYbQqWQRMmyecdo5VuSkYWkgZvcQXshB4vV2qkTb6AxbKruaNPfnMg4u2EA".into()).unwrap().into()
                    },
                    BlockHeader {
                        level: 2,
                        proto: 2,
                        predecessor: BlockHash::new("BMaBxGyVhtTiMKd7KA8HXJnbTK4e1TzffNc94G18op55HGQYVRk".into()).unwrap(),
                        timestamp: DateTime::parse_from_rfc3339("1970-01-01T00:00:00.002Z").unwrap().naive_utc(),
                        validation_pass: 2,
                        operations_hash: OperationListListHash::new("LLoaNF9sd5z2SZtSmUopYNX6qs77QAUJqrnd5ei378H4bcJhQcPt5".into()).unwrap(),
                        fitness: vec!["00000002ff000000020000000200000002".try_into().unwrap(), "00000002000000000200000002".try_into().unwrap()],
                        context: ContextHash::new("CoVj5HxwnPHpC1SgCC6pgqVPgw2vqFEqaC2bF5STqcbyX6giVrGn".into()).unwrap(),
                        payload_hash: BlockPayloadHash::new("vh2MHqgJtw8v7CDrZKYWtLmqGJtjzkRvs9yUeHNQqdgDJyCYm21q".into()).unwrap(),
                        payload_round: 2,
                        proof_of_work_nonce: "336ebf95efce0475".try_into().unwrap(),
                        seed_nonce_hash: Some(NonceHash::new("nceUeUCJRZ4M7FCSBsAUZU6dmxePdH7irje9Gfj9zWwCdfWd5B4Ee".into()).unwrap()),
                        liquidity_baking_escape_vote: false,
                        signature: GenericSignature::new("sigRsUhHqaFVBeV4qzyCZ6Y9TvoKajyNwyPQQCW3SbgPYY99MrpTqR2FopjzZEHMWoJG7LaTaHu7bnieKQRKqCRLA7hB7Ekp".into()).unwrap().into()
                    },
                )
                .into(),
                &hex!("03000000f1000000010114a98b361825acd1997319b0b01069908d1103df26a5646bf998cd6df80b95c60000000000000001018539ef2bf06ca139c6aeda9edc16c853f2b09ff232fab97d7a15150a602ea365000000110000000d00000001000000000100000001dc8d5cafd036ba185119ba904aefbdefd6d30de1f5e4a49fb20b0997ea2cdc357b08b37679350e62ea1bff3287d151c79156f0160b296bdade0ffa7f16f26b6300000001d4d34b5686c98ae100ff9d584824e3bf8b4817abdce782d94d93df6c60581e581990767cb8c0c07c577c328cddebd2da2433736411e17c2cfb282c8067e89c5a3e48246f50eca5e7525f000001260000000202f5043ad9d3aeea868db43f2abda52e1b7f176f928742964ce1db62d8f48cd67f0000000000000002028974da4dc7fcb31faab671f35d065db1d699a2b7d97bb830330977b8650591b0000000260000001100000002ff0000000200000002000000020000000d000000020000000002000000028e84ab5712175f8ab1ce14bcf5185d712c472a4e6abf51093a06c7e9042e59d258ef5ec7e36bb4004a4e7f10cb94032d59b65f8a86450c20a63d802ad749546200000002336ebf95efce0475ff37ad10c119adb450d7456104f3971536fb486124a262549c00d3310cd93e6820001dad11dad4d16f110476a24734b1414725506b354e01de4e54a4fdcec01604fda840b53f2cac4109c32680fe58600d96749b1d2891a0aa22b222ba36c864f001")
            ),
        ]
    }
}
