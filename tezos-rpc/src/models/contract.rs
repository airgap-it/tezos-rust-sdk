use {
    crate::serde_utils,
    num_bigint::BigInt,
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContractScript {
    pub code: Micheline,
    pub storage: Micheline,
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
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ContractScript>,
}
