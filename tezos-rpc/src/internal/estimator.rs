mod delegation;
mod origination;
mod register_global_constant;
mod reveal;
mod set_deposits_limit;
mod transaction;

use async_trait::async_trait;
use num_bigint::BigUint;
use tezos_core::types::mutez::Mutez;
use tezos_operation::operations::{Operation as TraitOperation, UnsignedOperation};

use crate::{
    client::TezosRpc,
    http::Http,
    models::{
        limits::{
            Limits, OperationLimits, BASE_FEE, FEE_PER_GAS_UNIT, FEE_PER_STORAGE_BYTE,
            FEE_SAFTY_MARGIN, GAS_SAFETY_MARGIN, STORAGE_SAFETY_MARGIN,
        },
        operation::{
            operation_result::{operations::InternalOperationResult, OperationResultStatus},
            OperationContent,
        },
    },
    Result,
};

#[async_trait]
pub trait FeeEstimator {
    async fn min_fee<'a>(
        &self,
        operation: UnsignedOperation,
        limits: &'a Limits,
    ) -> Result<UnsignedOperation>;
}

pub struct OperationFeeEstimator<HttpClient: Http> {
    rpc: TezosRpc<HttpClient>,
}

impl<HttpClient: Http> OperationFeeEstimator<HttpClient> {
    pub fn new(rpc: TezosRpc<HttpClient>) -> Self {
        Self { rpc }
    }
}

#[async_trait]
impl<HttpClient: Http + Send + Sync> FeeEstimator for OperationFeeEstimator<HttpClient> {
    async fn min_fee<'a>(
        &self,
        operation: UnsignedOperation,
        limits: &'a Limits,
    ) -> Result<UnsignedOperation> {
        let operation_with_limits = operation.apply(limits);
        let operation_contents = operation_with_limits.contents.clone();
        let operation = operation_with_limits.into();
        let run_operation_result = self.rpc.run_operation(&operation).send().await?;
        let unsigned_operation = UnsignedOperation::new(operation.branch, operation_contents);
        unsigned_operation.update_with(run_operation_result.contents)
    }
}

pub trait LimitsApplier {
    fn apply(self, limits: &Limits) -> Self;
}

impl LimitsApplier for UnsignedOperation {
    fn apply(self, limits: &Limits) -> Self {
        let max_limits = self.max_limits(limits);
        Self {
            branch: self.branch,
            contents: self
                .contents
                .into_iter()
                .map(|content| content.apply(None, &max_limits))
                .collect(),
        }
    }
}

trait FeeOperationLimitsApplier {
    fn apply(self, fee: Option<Mutez>, limits: &OperationLimits) -> Self;
}

impl FeeOperationLimitsApplier for tezos_operation::operations::OperationContent {
    fn apply(self, fee: Option<Mutez>, limits: &OperationLimits) -> Self {
        use tezos_operation::operations::{
            Delegation, Origination, RegisterGlobalConstant, Reveal, SetDepositsLimit, Transaction,
        };
        if self.has_fee() {
            return self;
        }
        match self {
            Self::Reveal(value) => Self::Reveal(Reveal {
                source: value.source,
                fee: fee.unwrap_or_default(),
                counter: value.counter,
                gas_limit: limits.gas.clone().into(),
                storage_limit: limits.storage.clone().into(),
                public_key: value.public_key,
            }),
            Self::Transaction(value) => Self::Transaction(Transaction {
                source: value.source,
                fee: fee.unwrap_or_default(),
                counter: value.counter,
                gas_limit: limits.gas.clone().into(),
                storage_limit: limits.storage.clone().into(),
                amount: value.amount,
                destination: value.destination,
                parameters: value.parameters,
            }),
            Self::Origination(value) => Self::Origination(Origination {
                source: value.source,
                fee: fee.unwrap_or_default(),
                counter: value.counter,
                gas_limit: limits.gas.clone().into(),
                storage_limit: limits.storage.clone().into(),
                balance: value.balance,
                delegate: value.delegate,
                script: value.script,
            }),
            Self::Delegation(value) => Self::Delegation(Delegation {
                source: value.source,
                fee: fee.unwrap_or_default(),
                counter: value.counter,
                gas_limit: limits.gas.clone().into(),
                storage_limit: limits.storage.clone().into(),
                delegate: value.delegate,
            }),
            Self::RegisterGlobalConstant(value) => {
                Self::RegisterGlobalConstant(RegisterGlobalConstant {
                    source: value.source,
                    fee: fee.unwrap_or_default(),
                    counter: value.counter,
                    gas_limit: limits.gas.clone().into(),
                    storage_limit: limits.storage.clone().into(),
                    value: value.value,
                })
            }
            Self::SetDepositsLimit(value) => Self::SetDepositsLimit(SetDepositsLimit {
                source: value.source,
                fee: fee.unwrap_or_default(),
                counter: value.counter,
                gas_limit: limits.gas.clone().into(),
                storage_limit: limits.storage.clone().into(),
                limit: value.limit,
            }),
            _ => self,
        }
    }
}

trait MaxLimits {
    fn max_limits(&self, limits: &Limits) -> OperationLimits;
}

impl<O: TraitOperation> MaxLimits for O {
    fn max_limits(&self, limits: &Limits) -> OperationLimits {
        let operation_gas_limit = operation_limits(self).gas;
        let available_gas_limit_per_block = if operation_gas_limit < limits.block.gas {
            limits.block.gas.clone() - operation_gas_limit
        } else {
            0u8.into()
        };
        let requires_estimation = self
            .contents()
            .iter()
            .filter(|content| content.has_fee())
            .collect::<Vec<_>>()
            .len();
        let max_gas_limit_per_operation: BigUint = if requires_estimation > 0 {
            available_gas_limit_per_block / requires_estimation
        } else {
            0u8.into()
        };

        OperationLimits {
            gas: limits
                .operation
                .clone()
                .gas
                .min(max_gas_limit_per_operation),
            storage: limits.operation.storage.clone(),
        }
    }
}

fn operation_limits<O: TraitOperation>(operation: &O) -> OperationLimits {
    operation
        .contents()
        .iter()
        .fold(OperationLimits::zero(), |acc, operation_content| {
            let limit = operation_content_limits(operation_content);
            OperationLimits {
                gas: acc.gas + limit.gas,
                storage: acc.storage + limit.storage,
            }
        })
}

fn operation_content_limits(
    operation_content: &tezos_operation::operations::OperationContent,
) -> OperationLimits {
    use tezos_operation::operations::OperationContent;

    if !operation_content.has_fee() {
        return OperationLimits::zero();
    }
    match operation_content {
        OperationContent::Reveal(value) => OperationLimits {
            gas: value.gas_limit.clone().into(),
            storage: value.storage_limit.clone().into(),
        },
        OperationContent::Transaction(value) => OperationLimits {
            gas: value.gas_limit.clone().into(),
            storage: value.storage_limit.clone().into(),
        },
        OperationContent::Origination(value) => OperationLimits {
            gas: value.gas_limit.clone().into(),
            storage: value.storage_limit.clone().into(),
        },
        OperationContent::Delegation(value) => OperationLimits {
            gas: value.gas_limit.clone().into(),
            storage: value.storage_limit.clone().into(),
        },
        OperationContent::RegisterGlobalConstant(value) => OperationLimits {
            gas: value.gas_limit.clone().into(),
            storage: value.storage_limit.clone().into(),
        },
        OperationContent::SetDepositsLimit(value) => OperationLimits {
            gas: value.gas_limit.clone().into(),
            storage: value.storage_limit.clone().into(),
        },
        _ => OperationLimits::zero(),
    }
}

trait UpdateWith<T> {
    fn update_with(self, value: T) -> Result<Self>
    where
        Self: Sized;
}

impl UpdateWith<Vec<OperationContent>> for UnsignedOperation {
    fn update_with(self, value: Vec<OperationContent>) -> Result<Self> {
        Ok(Self {
            branch: self.branch,
            contents: self
                .contents
                .into_iter()
                .zip(value)
                .map(|(content, rpc_content)| content.update_with(rpc_content))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl UpdateWith<OperationContent> for tezos_operation::operations::OperationContent {
    fn update_with(self, rpc_operation_content: OperationContent) -> Result<Self> {
        if !operation_content_matches(&self, &rpc_operation_content) {
            return Ok(self);
        }

        if let Some(metadata_limits) = rpc_operation_content.metadata_limits() {
            let forged = (&self).to_forged_bytes()?;
            let size = forged.len() + 32 + 64; // content size + forged branch size + forged signature size
            let fee = fee(size, &metadata_limits)?;
            return Ok(self.apply(Some(fee), &metadata_limits));
        }

        return Ok(self);
    }
}

fn fee(operation_size: usize, limits: &OperationLimits) -> Result<Mutez> {
    let gas_fee: Mutez = (BigUint::from(FEE_PER_GAS_UNIT) * limits.gas.clone()).try_into()?;
    let storage_fee: Mutez =
        (BigUint::from(FEE_PER_STORAGE_BYTE) * BigUint::from(operation_size)).try_into()?;
    let base: Mutez = BASE_FEE.into();
    let safty_margin: Mutez = FEE_SAFTY_MARGIN.into();

    Ok(base + gas_fee + storage_fee + safty_margin)
}

fn operation_content_matches(
    operation_content: &tezos_operation::operations::OperationContent,
    rpc_operation_content: &OperationContent,
) -> bool {
    match (operation_content, rpc_operation_content) {
        (tezos_operation::operations::OperationContent::Reveal(_), OperationContent::Reveal(_))
        | (
            tezos_operation::operations::OperationContent::Transaction(_),
            OperationContent::Transaction(_),
        )
        | (
            tezos_operation::operations::OperationContent::Origination(_),
            OperationContent::Origination(_),
        )
        | (
            tezos_operation::operations::OperationContent::Delegation(_),
            OperationContent::Delegation(_),
        )
        | (
            tezos_operation::operations::OperationContent::RegisterGlobalConstant(_),
            OperationContent::RegisterGlobalConstant(_),
        )
        | (
            tezos_operation::operations::OperationContent::SetDepositsLimit(_),
            OperationContent::SetDepositsLimit(_),
        ) => true,
        _ => false,
    }
}

impl OperationContent {
    fn metadata_limits(&self) -> Option<OperationLimits> {
        match self {
            Self::Reveal(value) => value.metadata.as_ref().map(|metadata| metadata.limits()),
            Self::Transaction(value) => value.metadata.as_ref().map(|metadata| metadata.limits()),
            Self::Origination(value) => value.metadata.as_ref().map(|metadata| metadata.limits()),
            Self::Delegation(value) => value.metadata.as_ref().map(|metadata| metadata.limits()),
            Self::RegisterGlobalConstant(value) => {
                value.metadata.as_ref().map(|metadata| metadata.limits())
            }
            Self::SetDepositsLimit(value) => {
                value.metadata.as_ref().map(|metadata| metadata.limits())
            }
            _ => None,
        }
    }
}

trait RpcMetadata<OperationResult: RpcOperationResult> {
    fn operation_result(&self) -> &OperationResult;
    fn internal_operation_results(&self) -> Option<&Vec<InternalOperationResult>>;

    fn limits(&self) -> OperationLimits {
        if let Some(results) = self.internal_operation_results() {
            let operation_result_limits = self.operation_result().limits();

            results
                .into_iter()
                .fold(operation_result_limits, |acc, result| {
                    let limits = result.limits();

                    OperationLimits {
                        gas: acc.gas + limits.gas,
                        storage: acc.storage + limits.storage,
                    }
                })
        } else {
            self.operation_result().limits()
        }
    }
}

impl InternalOperationResult {
    fn limits(&self) -> OperationLimits {
        match self {
            Self::Transaction(value) => value
                .result
                .as_ref()
                .map_or(OperationLimits::zero(), |result| result.limits()),
            Self::Origination(value) => value.result.limits(),
            Self::Delegation(value) => value.result.limits(),
        }
    }
}

trait RpcOperationResult {
    fn status(&self) -> OperationResultStatus;
    fn number_of_originated_contracts(&self) -> usize;
    fn consumed_gas(&self) -> BigUint;
    fn consumed_milligas(&self) -> BigUint;
    fn paid_storage_size_diff(&self) -> Option<BigUint>;
    fn allocated_destination_contract(&self) -> Option<bool>;

    fn limits(&self) -> OperationLimits {
        OperationLimits {
            gas: self.consumed_gas() + GAS_SAFETY_MARGIN,
            storage: self.paid_storage_size_diff().unwrap_or(0u8.into())
                + self.burn_fee()
                + STORAGE_SAFETY_MARGIN,
        }
    }

    fn burn_fee(&self) -> BigUint {
        let mut sum: BigUint = 0u8.into();
        if let Some(allocated) = self.allocated_destination_contract() {
            if allocated {
                sum += 257u16;
            }
        }

        sum += (self.number_of_originated_contracts() as u64) * 257;

        return sum;
    }
}