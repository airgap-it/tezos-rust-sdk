use {
    crate::{
        models::balance_update::BalanceUpdate, models::operation::kind::OperationKind,
        models::operation::operation_result::operations::reveal::RevealOperationResult,
    },
    crate::{Error, Result},
    serde::{Deserialize, Serialize},
    tezos_core::types::{
        encoded::{ImplicitAddress, PublicKey},
        mutez::Mutez,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reveal {
    /// [OperationKind::Reveal]
    pub kind: OperationKind,
    /// Public key hash (Base58Check-encoded)
    pub source: ImplicitAddress,
    pub fee: Mutez,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    /// Public key (Base58Check-encoded)
    pub public_key: PublicKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RevealMetadata>,
}

impl From<tezos_operation::operations::Reveal> for Reveal {
    fn from(value: tezos_operation::operations::Reveal) -> Self {
        Self {
            kind: OperationKind::Reveal,
            source: value.source,
            fee: value.fee,
            counter: value.counter.into(),
            gas_limit: value.gas_limit.into(),
            storage_limit: value.storage_limit.into(),
            public_key: value.public_key,
            metadata: None,
        }
    }
}

impl TryFrom<Reveal> for tezos_operation::operations::Reveal {
    type Error = Error;

    fn try_from(value: Reveal) -> Result<Self> {
        Ok(Self {
            source: value.source,
            fee: value.fee,
            counter: value.counter.try_into()?,
            gas_limit: value.gas_limit.try_into()?,
            storage_limit: value.storage_limit.try_into()?,
            public_key: value.public_key,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevealMetadata {
    pub operation_result: RevealOperationResult,
    #[serde(default)]
    pub balance_updates: Vec<BalanceUpdate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::operation::kind::OperationKind;

    #[test]
    fn test_serialize() {
        let reveal: Reveal = Reveal {
            kind: OperationKind::Reveal,
            source: "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".try_into().unwrap(),
            fee: "111111111111111111".try_into().unwrap(),
            counter: "222222222222222222222222222222222".to_string(),
            gas_limit: "333333333333333333333333333333333".to_string(),
            storage_limit: "444444444444444444444444444444".to_string(),
            public_key: "edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg"
                .try_into()
                .unwrap(),
            metadata: None,
        };

        let expected = "{\"kind\":\"reveal\",\"source\":\"tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL\",\"fee\":\"111111111111111111\",\"counter\":\"222222222222222222222222222222222\",\"gas_limit\":\"333333333333333333333333333333333\",\"storage_limit\":\"444444444444444444444444444444\",\"public_key\":\"edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg\"}";
        let received = serde_json::to_string(&reveal).unwrap();
        assert_eq!(expected, received)
    }

    #[test]
    fn test_deserialize() {
        let expected_reveal: Reveal = Reveal {
            kind: OperationKind::Reveal,
            source: "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL".try_into().unwrap(),
            fee: "111111111111111111".try_into().unwrap(),
            counter: "222222222222222222222222222222222".to_string(),
            gas_limit: "333333333333333333333333333333333".to_string(),
            storage_limit: "444444444444444444444444444444".to_string(),
            public_key: "edpku6hZd7SmkEW2YNJ5iJDUw7PbqpS58hRJJWVWhaZtGcXr9XrKCg"
                .try_into()
                .unwrap(),
            metadata: None,
        };
        let value = serde_json::json!({
            "kind": "reveal",
            "source": "tz1bLUuUBWtJqFX2Hz3A3whYE5SNTAGHjcpL",
            "fee": "111111111111111111",
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
