pub mod transaction;
pub mod reveal;
pub mod delegation;
pub mod set_deposits_limit;
pub mod origination;
pub mod register_global_constant;

use {
    self::transaction::{TransactionOperationResult, TransactionSuccessfulManagerOperationResult},
    self::reveal::{RevealOperationResult, RevealSuccessfulManagerOperationResult},
    self::delegation::{DelegationOperationResult, DelegationSuccessfulManagerOperationResult},
    self::set_deposits_limit::{SetDepositsLimitOperationResult, SetDepositsLimitSuccessfulManagerOperationResult},
    self::origination::{OriginationOperationResult, OriginationSuccessfulManagerOperationResult},
    self::register_global_constant::RegisterGlobalConstantOperationResult,
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
