pub mod kind;
pub mod metadata;
pub mod operation_contents_and_result;
pub mod operation_result;

use {
    self::{
        operation_contents_and_result::activate_account::ActivateAccount,
        operation_contents_and_result::ballot::Ballot,
        operation_contents_and_result::delegation::Delegation,
        operation_contents_and_result::double_baking_evidence::DoubleBakingEvidence,
        operation_contents_and_result::double_endorsement_evidence::DoubleEndorsementEvidence,
        operation_contents_and_result::double_preendorsement_evidence::DoublePreendorsementEvidence,
        operation_contents_and_result::endorsement::Endorsement,
        operation_contents_and_result::failing_noop::FailingNoop,
        operation_contents_and_result::origination::Origination,
        operation_contents_and_result::preendorsement::Preendorsement,
        operation_contents_and_result::proposals::Proposals,
        operation_contents_and_result::register_global_constant::RegisterGlobalConstant,
        operation_contents_and_result::reveal::Reveal,
        operation_contents_and_result::seed_nonce_revelation::SeedNonceRevelation,
        operation_contents_and_result::set_deposits_limit::SetDepositsLimit,
        operation_contents_and_result::transaction::Transaction,
        operation_contents_and_result::transfer_ticket::TransferTicket,
        operation_contents_and_result::tx_rollup_commit::TxRollupCommit,
        operation_contents_and_result::tx_rollup_dispatch_tickets::TxRollupDispatchTickets,
        operation_contents_and_result::tx_rollup_finalize_commitment::TxRollupFinalizeCommitment,
        operation_contents_and_result::tx_rollup_origination::TxRollupOrigination,
        operation_contents_and_result::tx_rollup_rejection::TxRollupRejection,
        operation_contents_and_result::tx_rollup_remove_commitment::TxRollupRemoveCommitment,
        operation_contents_and_result::tx_rollup_return_bond::TxRollupReturnBond,
        operation_contents_and_result::tx_rollup_submit_batch::TxRollupSubmitBatch,
    },
    crate::{Error, Result},
    serde::{Deserialize, Serialize},
    tezos_core::types::encoded::{BlockHash, ChainId, OperationHash, ProtocolHash, Signature},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ProtocolHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<ChainId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<OperationHash>,
    pub branch: BlockHash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
    pub contents: Vec<OperationContent>,
}

impl From<tezos_operation::operations::UnsignedOperation> for Operation {
    fn from(value: tezos_operation::operations::UnsignedOperation) -> Self {
        Self {
            protocol: None,
            chain_id: None,
            hash: None,
            branch: value.branch,
            signature: None,
            contents: value
                .contents
                .into_iter()
                .map(|content| content.into())
                .collect(),
        }
    }
}

impl From<tezos_operation::operations::SignedOperation> for Operation {
    fn from(value: tezos_operation::operations::SignedOperation) -> Self {
        Self {
            protocol: None,
            chain_id: None,
            hash: None,
            branch: value.branch,
            signature: Some(value.signature),
            contents: value
                .contents
                .into_iter()
                .map(|content| content.into())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OperationContent {
    // Present in alpha protocol
    Endorsement(Endorsement),
    Preendorsement(Preendorsement),
    SeedNonceRevelation(SeedNonceRevelation),
    DoubleEndorsementEvidence(DoubleEndorsementEvidence),
    DoublePreendorsementEvidence(DoublePreendorsementEvidence),
    ActivateAccount(ActivateAccount),
    Proposals(Proposals),
    Ballot(Ballot),
    Reveal(Reveal),
    Transaction(Transaction),
    Origination(Origination),
    Delegation(Delegation),
    RegisterGlobalConstant(RegisterGlobalConstant),
    SetDepositsLimit(SetDepositsLimit),
    FailingNoop(FailingNoop),
    // Removed in hangzhou protocol (https://tezos.gitlab.io/protocols/tenderbake.html)
    DoubleBakingEvidence(DoubleBakingEvidence),
    // Added in Jakarta
    TxRollupOrigination(TxRollupOrigination),
    TxRollupSubmitBatch(TxRollupSubmitBatch),
    TxRollupCommit(TxRollupCommit),
    TxRollupReturnBond(TxRollupReturnBond),
    TxRollupFinalizeCommitment(TxRollupFinalizeCommitment),
    TxRollupRemoveCommitment(TxRollupRemoveCommitment),
    TxRollupRejection(TxRollupRejection),
    TransferTicket(TransferTicket),
    TxRollupDispatchTickets(TxRollupDispatchTickets),
    Unknown(serde_json::Value), // must be the last one
}

impl From<tezos_operation::operations::OperationContent> for OperationContent {
    fn from(value: tezos_operation::operations::OperationContent) -> Self {
        match value {
            tezos_operation::operations::OperationContent::SeedNonceRevelation(value) => {
                Self::SeedNonceRevelation(value.into())
            }
            tezos_operation::operations::OperationContent::DoubleEndorsementEvidence(value) => {
                Self::DoubleEndorsementEvidence(value.into())
            }
            tezos_operation::operations::OperationContent::DoubleBakingEvidence(value) => {
                Self::DoubleBakingEvidence(value.into())
            }
            tezos_operation::operations::OperationContent::ActivateAccount(value) => {
                Self::ActivateAccount(value.into())
            }
            tezos_operation::operations::OperationContent::Proposals(value) => {
                Self::Proposals(value.into())
            }
            tezos_operation::operations::OperationContent::Ballot(value) => {
                Self::Ballot(value.into())
            }
            tezos_operation::operations::OperationContent::DoublePreendorsementEvidence(value) => {
                Self::DoublePreendorsementEvidence(value.into())
            }
            tezos_operation::operations::OperationContent::FailingNoop(value) => {
                Self::FailingNoop(value.into())
            }
            tezos_operation::operations::OperationContent::Preendorsement(value) => {
                Self::Preendorsement(value.into())
            }
            tezos_operation::operations::OperationContent::Endorsement(value) => {
                Self::Endorsement(value.into())
            }
            tezos_operation::operations::OperationContent::Reveal(value) => {
                Self::Reveal(value.into())
            }
            tezos_operation::operations::OperationContent::Transaction(value) => {
                Self::Transaction(value.into())
            }
            tezos_operation::operations::OperationContent::Origination(value) => {
                Self::Origination(value.into())
            }
            tezos_operation::operations::OperationContent::Delegation(value) => {
                Self::Delegation(value.into())
            }
            tezos_operation::operations::OperationContent::RegisterGlobalConstant(value) => {
                Self::RegisterGlobalConstant(value.into())
            }
            tezos_operation::operations::OperationContent::SetDepositsLimit(value) => {
                Self::SetDepositsLimit(value.into())
            }
        }
    }
}

impl TryFrom<OperationContent> for tezos_operation::operations::OperationContent {
    type Error = Error;

    fn try_from(value: OperationContent) -> Result<Self> {
        match value {
            OperationContent::Endorsement(value) => Ok(Self::Endorsement(value.try_into()?)),
            OperationContent::Preendorsement(value) => Ok(Self::Preendorsement(value.into())),
            OperationContent::SeedNonceRevelation(value) => {
                Ok(Self::SeedNonceRevelation(value.try_into()?))
            }
            OperationContent::DoubleEndorsementEvidence(value) => {
                Ok(Self::DoubleEndorsementEvidence(value.try_into()?))
            }
            OperationContent::DoublePreendorsementEvidence(value) => {
                Ok(Self::DoublePreendorsementEvidence(value.try_into()?))
            }
            OperationContent::ActivateAccount(value) => {
                Ok(Self::ActivateAccount(value.try_into()?))
            }
            OperationContent::Proposals(value) => Ok(Self::Proposals(value.into())),
            OperationContent::Ballot(value) => Ok(Self::Ballot(value.into())),
            OperationContent::Reveal(value) => Ok(Self::Reveal(value.try_into()?)),
            OperationContent::Transaction(value) => Ok(Self::Transaction(value.try_into()?)),
            OperationContent::Origination(value) => Ok(Self::Origination(value.try_into()?)),
            OperationContent::Delegation(value) => Ok(Self::Delegation(value.try_into()?)),
            OperationContent::RegisterGlobalConstant(value) => {
                Ok(Self::RegisterGlobalConstant(value.try_into()?))
            }
            OperationContent::SetDepositsLimit(value) => {
                Ok(Self::SetDepositsLimit(value.try_into()?))
            }
            OperationContent::FailingNoop(value) => Ok(Self::FailingNoop(value.try_into()?)),
            OperationContent::DoubleBakingEvidence(value) => {
                Ok(Self::DoubleBakingEvidence(value.try_into()?))
            }
            OperationContent::TxRollupOrigination(_)
            | OperationContent::TxRollupSubmitBatch(_)
            | OperationContent::TxRollupCommit(_)
            | OperationContent::TxRollupReturnBond(_)
            | OperationContent::TxRollupFinalizeCommitment(_)
            | OperationContent::TxRollupRemoveCommitment(_)
            | OperationContent::TxRollupRejection(_)
            | OperationContent::TransferTicket(_)
            | OperationContent::TxRollupDispatchTickets(_)
            | OperationContent::Unknown(_) => Err(Error::OperationNotSupported),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationWithMetadata {
    pub contents: Vec<OperationContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
}
