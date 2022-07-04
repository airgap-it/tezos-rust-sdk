pub mod delegation;
pub mod origination;
pub mod register_global_constant;
pub mod reveal;
pub mod set_deposits_limit;
pub mod transaction;

use {
    self::delegation::{DelegationOperationResult, DelegationSuccessfulManagerOperationResult},
    self::origination::{OriginationOperationResult, OriginationSuccessfulManagerOperationResult},
    self::register_global_constant::RegisterGlobalConstantOperationResult,
    self::reveal::{RevealOperationResult, RevealSuccessfulManagerOperationResult},
    self::set_deposits_limit::{
        SetDepositsLimitOperationResult, SetDepositsLimitSuccessfulManagerOperationResult,
    },
    self::transaction::{TransactionOperationResult, TransactionSuccessfulManagerOperationResult},
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
