use {
    super::Kind,
    crate::models::operation::operation_result::DiffAction,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SaplingState {
    pub kind: Kind,
    pub id: String,
    pub diff: Diff,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Diff {
    pub action: DiffAction,
    pub updates: Vec<Update>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub memo_size: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Update {
    pub commitments_and_ciphertexts: Vec<[CommitmentAndCiphertext; 2]>,
    pub nullifiers: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum CommitmentAndCiphertext {
    Commitment(String),
    Ciphertext(Ciphertext),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Ciphertext {
    pub cv: String,
    pub epk: String,
    pub payload_enc: String,
    pub nonce_enc: String,
    pub payload_out: String,
    pub nonce_out: String,
}
