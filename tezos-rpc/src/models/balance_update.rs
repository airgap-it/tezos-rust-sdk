use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum BalanceUpdate {
    Contract(Contract),
    Categorized(CategorizedBalanceUpdate),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Contract,
    Freezer,
    Accumulator,
    Minted,
    Burned,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Rewards,
    Fees,
    Deposits,
    LegacyRewards,
    #[serde(alias = "block fees")]
    BlockFees,
    LegacyDeposits,
    LegacyFees,
    #[serde(alias = "nonce revelation rewards")]
    NonceRevelationRewards,
    #[serde(alias = "double signing evidence rewards")]
    DoubleSigningEvidenceRewards,
    #[serde(alias = "endorsing rewards")]
    EndorsingRewards,
    #[serde(alias = "baking rewards")]
    BakingRewards,
    #[serde(alias = "baking bonuses")]
    BakingBonuses,
    #[serde(alias = "storage fees")]
    StorageFees,
    Punishments,
    #[serde(alias = "lost endorsing rewards")]
    LostEndorsingRewards,
    Subsidy,
    Burned,
    Commitment,
    Bootstrap,
    Invoice,
    Minted,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    Block,
    Migration,
    Subsidy,
    Simulation,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contract {
    pub kind: Kind,
    pub change: String,
    pub contract: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CategorizedBalanceUpdate {
    pub kind: Kind,
    pub change: String,
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revelation: Option<bool>,
}
