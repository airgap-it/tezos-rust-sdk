use {
    derive_more::Display,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    Endorsement,
    Preendorsement,
    SeedNonceRevelation,
    DoubleEndorsementEvidence,
    DoublePreendorsementEvidence,
    DoubleBakingEvidence,
    ActivateAccount,
    Proposals,
    Ballot,
    Reveal,
    Transaction,
    Origination,
    Delegation,
    RegisterGlobalConstant,
    SetDepositsLimit,
    FailingNoop,
}
