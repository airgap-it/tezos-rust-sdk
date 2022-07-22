use num_bigint::BigUint;

pub const GAS_SAFETY_MARGIN: u64 = 100;
pub const STORAGE_SAFETY_MARGIN: u64 = 100;
pub const FEE_SAFTY_MARGIN: u8 = 100;
pub const BASE_FEE: u8 = 100;
pub const FEE_PER_GAS_UNIT: u64 = 100; // nanoTez
pub const FEE_PER_STORAGE_BYTE: u64 = 1000; // nanoTez
pub const FEE_PER_PAID_STORAGE_SIZE_DIFF_BYTE: u64 = 250; // mutez
pub const ALLOCATION_FEE: u64 = 64250;
pub const NANO_TEZ_PER_MUTEZ: u64 = 1000;
pub const HARD_GAS_LIMIT_PER_OPERATION: u64 = 1040000;
pub const HARD_GAS_LIMIT_PER_BLOCK: u64 = 5200000;
pub const HARD_STORAGE_LIMIT_PER_OPERATION: u64 = 60000;

#[derive(Debug, Clone)]
pub struct Limits {
    pub operation: OperationLimits,
    pub block: BlockLimits,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            operation: Default::default(),
            block: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationLimits {
    pub gas: BigUint,
    pub storage: BigUint,
}

impl OperationLimits {
    pub fn zero() -> Self {
        OperationLimits {
            gas: 0u8.into(),
            storage: 0u8.into(),
        }
    }
}

impl Default for OperationLimits {
    fn default() -> Self {
        Self {
            gas: HARD_GAS_LIMIT_PER_OPERATION.into(),
            storage: HARD_STORAGE_LIMIT_PER_OPERATION.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockLimits {
    pub gas: BigUint,
}

impl Default for BlockLimits {
    fn default() -> Self {
        Self {
            gas: HARD_GAS_LIMIT_PER_BLOCK.into(),
        }
    }
}
