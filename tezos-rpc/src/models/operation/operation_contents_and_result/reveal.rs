use {
    crate::{
        models::balance_update::BalanceUpdate,
        models::operation::kind::OperationKind,
        models::operation::operation_result::operations::reveal::RevealOperationResult
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reveal {
    /// [OperationKind::Reveal]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    /// Public key (Base58Check-encoded)
    pub public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RevealMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevealMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub operation_result: RevealOperationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<Vec<InternalRevealOperationResult>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalRevealOperationResult {
    /// [OperationKind::Reveal]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: String,
    /// integer ∈ [0, 2^16-1]
    pub nonce: u16,
    /// Public key (Base58Check-encoded)
    pub public_key: String,
    pub result: RevealOperationResult,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::operation::kind::OperationKind;

    #[test]
    fn test_serialize() {
        let reveal: Reveal = Reveal {
            kind: OperationKind::Reveal,
            source: "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".to_string(),
            fee: "111111111111111111111111111111111".to_string(),
            counter: "222222222222222222222222222222222".to_string(),
            gas_limit: "333333333333333333333333333333333".to_string(),
            storage_limit: "444444444444444444444444444444".to_string(),
            public_key: "edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg".to_string(),
            metadata: None,
        };

        let expected = "{\"kind\":\"reveal\",\"source\":\"tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL\",\"fee\":\"111111111111111111111111111111111\",\"counter\":\"222222222222222222222222222222222\",\"gas_limit\":\"333333333333333333333333333333333\",\"storage_limit\":\"444444444444444444444444444444\",\"public_key\":\"edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg\"}";
        let received = serde_json::to_string(&reveal).unwrap();
        assert_eq!(expected, received)
    }

    #[test]
    fn test_deserialize() {
        let expected_reveal: Reveal = Reveal {
            kind: OperationKind::Reveal,
            source: "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".to_string(),
            fee: "111111111111111111111111111111111".to_string(),
            counter: "222222222222222222222222222222222".to_string(),
            gas_limit: "333333333333333333333333333333333".to_string(),
            storage_limit: "444444444444444444444444444444".to_string(),
            public_key: "edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg".to_string(),
            metadata: None,
        };
        let value = serde_json::json!({
            "kind": "reveal",
            "source": "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL",
            "fee": "111111111111111111111111111111111",
            "counter": "222222222222222222222222222222222",
            "gas_limit": "333333333333333333333333333333333",
            "storage_limit": "444444444444444444444444444444",
            "public_key": "edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg"
        });

        let reveal = serde_json::from_value::<Reveal>(value).unwrap();
        assert_eq!(expected_reveal.kind, reveal.kind);
        assert_eq!(expected_reveal.source, reveal.source);
        assert_eq!(expected_reveal.fee, reveal.fee);
        assert_eq!(expected_reveal.counter, reveal.counter);
        assert_eq!(expected_reveal.gas_limit, reveal.gas_limit);
        assert_eq!(expected_reveal.storage_limit, reveal.storage_limit);
        assert_eq!(expected_reveal.public_key, reveal.public_key);
    }
}
