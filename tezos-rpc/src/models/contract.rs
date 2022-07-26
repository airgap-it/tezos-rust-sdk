use tezos_core::types::encoded::ImplicitAddress;

use {
    crate::serde_utils,
    num_bigint::BigInt,
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    tezos_michelson::micheline::{sequence::Sequence, Micheline},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContractScript {
    pub code: Sequence,
    pub storage: Micheline,
}

impl From<tezos_operation::operations::Script> for ContractScript {
    fn from(value: tezos_operation::operations::Script) -> Self {
        Self {
            code: value.code,
            storage: value.storage,
        }
    }
}

impl From<ContractScript> for tezos_operation::operations::Script {
    fn from(value: ContractScript) -> Self {
        Self {
            code: value.code,
            storage: value.storage,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    #[serde(deserialize_with = "serde_utils::number_of_string")]
    pub balance: BigInt,
    #[serde(
        default,
        deserialize_with = "serde_utils::option_number_of_option_string"
    )]
    pub counter: Option<BigInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<ImplicitAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ContractScript>,
}

#[derive(Clone, Copy, Serialize)]
pub enum UnparsingMode {
    Readable,
    Optimized,
    #[allow(non_camel_case_types)] // This format is expected by the RPC
    Optimized_legacy,
}

impl Default for UnparsingMode {
    fn default() -> Self {
        Self::Optimized
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContractEntrypoints {
    pub entrypoints: HashMap<String, Micheline>,
}
