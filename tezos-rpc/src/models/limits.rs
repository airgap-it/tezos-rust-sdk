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
