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

use tezos_core::types::encoded::{BlockHash, Signature};

pub use self::{
    activate_account::ActivateAccount,
    ballot::Ballot,
    delegation::Delegation,
    double_baking_evidence::DoubleBakingEvidence,
    double_endorsement_evidence::DoubleEndorsementEvidence,
    double_preendorsement_evidence::DoublePreendorsementEvidence,
    endorsement::Endorsement,
    failing_noop::FailingNoop,
    origination::Origination,
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
    transaction::Transaction,
};

pub trait Operation {
    fn branch(&self) -> &BlockHash;
    fn contents(&self) -> &[OperationContent];
}

pub struct UnsignedOperation {
    branch: BlockHash,
    contents: Vec<OperationContent>,
}

impl UnsignedOperation {
    pub fn new(branch: BlockHash, contents: Vec<OperationContent>) -> Self {
        Self { branch, contents }
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

#[repr(u8)]
enum OperationContentTag {
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
}

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
}

mod traits {
    use tezos_core::types::{
        encoded::{BlockPayloadHash, ImplicitAddress},
        mutez::Mutez,
        number::Nat,
    };

    pub trait OperationContent {
        fn tag() -> &'static [u8];
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
