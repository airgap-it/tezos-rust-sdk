use {
    crate::types::timestamp::Timestamp,
    serde::{self, Deserialize, Deserializer, Serializer},
};

const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

pub fn serialize<S>(date: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Timestamp::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}
