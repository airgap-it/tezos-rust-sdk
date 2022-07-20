pub mod delegation;
pub mod origination;
pub mod register_global_constant;
pub mod reveal;
pub mod set_deposits_limit;
pub mod transaction;
pub mod tx_rollup_commit;
pub mod tx_rollup_finalize_commitment;
pub mod tx_rollup_origination;
pub mod tx_rollup_rejection;
pub mod tx_rollup_remove_commitment;
pub mod tx_rollup_return_bond;
pub mod tx_rollup_submit_batch;

use {
    self::delegation::{
        DelegationOperationResult, DelegationSuccessfulManagerOperationResult,
        InternalDelegationOperationResult,
    },
    self::origination::{
        InternalOriginationOperationResult, OriginationOperationResult,
        OriginationSuccessfulManagerOperationResult,
    },
    self::register_global_constant::RegisterGlobalConstantOperationResult,
    self::reveal::{RevealOperationResult, RevealSuccessfulManagerOperationResult},
    self::set_deposits_limit::{
        SetDepositsLimitOperationResult, SetDepositsLimitSuccessfulManagerOperationResult,
    },
    self::transaction::{
        InternalTransactionOperationResult, TransactionOperationResult,
        TransactionSuccessfulManagerOperationResult,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum OperationResult {
    Reveal(RevealOperationResult),
    Transaction(TransactionOperationResult),
    Origination(OriginationOperationResult),
    Delegation(DelegationOperationResult),
    RegisterGlobalConstant(RegisterGlobalConstantOperationResult),
    SetDepositsLimits(SetDepositsLimitOperationResult),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SuccessfulManagerOperationResult {
    Transaction(TransactionSuccessfulManagerOperationResult),
    Origination(OriginationSuccessfulManagerOperationResult),
    Reveal(RevealSuccessfulManagerOperationResult),
    Delegation(DelegationSuccessfulManagerOperationResult),
    SetDepositsLimits(SetDepositsLimitSuccessfulManagerOperationResult),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum InternalOperationResult {
    Transaction(InternalTransactionOperationResult),
    Origination(InternalOriginationOperationResult),
    Delegation(InternalDelegationOperationResult),
}
