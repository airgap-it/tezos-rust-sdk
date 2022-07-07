use num_bigint::BigInt;

pub const GAS_SAFETY_MARGIN: u64 = 100;
pub const STORAGE_SAFETY_MARGIN: u64 = 100;
pub const FEE_SAFTY_MARGIN: u64 = 100;
pub const BASE_FEE: u64 = 100;
pub const FEE_PER_GAS_UNIT: u64 = 100; // nanoTez
pub const FEE_PER_STORAGE_BYTE: u64 = 1000; // nanoTez
pub const FEE_PER_PAID_STORAGE_SIZE_DIFF_BYTE: u64 = 250; // mutez
pub const ALLOCATION_FEE: u64 = 64250;
pub const NANO_TEZ_PER_MUTEZ: u64 = 1000;
pub const HARD_GAS_LIMIT_PER_OPERATION: u64 = 1040000;
pub const HARD_GAS_LIMIT_PER_BLOCK: u64 = 5200000;
pub const HARD_STORAGE_LIMIT_PER_OPERATION: u64 = 60000;

fn nanotez_to_mutez(value: u64) -> u64 {
    if value % NANO_TEZ_PER_MUTEZ == 0 {
        return value / NANO_TEZ_PER_MUTEZ;
    }

    (value / NANO_TEZ_PER_MUTEZ) + 1
}

#[derive(Debug)]
pub struct Fee {
    pub value: BigInt,
    pub limits: Limits,
}

impl Fee {
    pub fn new(value: BigInt, gas_limit: u64, storage_limit: u64) -> Self {
        Fee {
            value,
            limits: Limits {
                gas_limit,
                storage_limit,
            },
        }
    }

    pub fn zero() -> Self {
        Fee {
            value: 0.into(),
            limits: Limits {
                gas_limit: 0,
                storage_limit: 0,
            },
        }
    }

    pub fn zero_with_max_limits() -> Self {
        Fee {
            value: 0.into(),
            limits: Limits::default(),
        }
    }

    pub fn zero_with_limits(limits: Option<Limits>) -> Self {
        Fee {
            value: 0.into(),
            limits: limits.unwrap_or_default(),
        }
    }
}

impl From<(Limits, u64)> for Fee {
    fn from((limits, operation_size): (Limits, u64)) -> Self {
        let gas_fee = nanotez_to_mutez(limits.gas_limit * FEE_PER_GAS_UNIT);
        let storage_fee = nanotez_to_mutez(operation_size * FEE_PER_STORAGE_BYTE);

        Fee {
            value: (BASE_FEE + gas_fee + storage_fee + FEE_SAFTY_MARGIN).into(),
            limits,
        }
    }
}

impl Default for Fee {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub gas_limit: u64,
    pub storage_limit: u64,
}

impl Limits {
    pub fn zero() -> Self {
        Limits {
            gas_limit: 0,
            storage_limit: 0,
        }
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            gas_limit: HARD_GAS_LIMIT_PER_OPERATION,
            storage_limit: HARD_STORAGE_LIMIT_PER_OPERATION,
        }
    }
}
