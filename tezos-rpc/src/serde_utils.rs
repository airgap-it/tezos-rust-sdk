use {
    serde::{Deserialize, Deserializer},
    std::str::FromStr,
};

pub(crate) mod rfc3339_timestamp {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt = DateTime::<Utc>::from_utc(*date, Utc);
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|op| op.naive_utc())
            .map_err(serde::de::Error::custom)
    }
}

/// Deserialize `String` into `T`.
pub fn number_of_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Format<T> {
        String(String),
        Number(T),
    }

    match Format::<T>::deserialize(deserializer)? {
        Format::String(v) => v.parse::<T>().map_err(serde::de::Error::custom),
        Format::Number(v) => Ok(v),
    }
}

/// Deserialize possible `Option<String>` into `Option<T>`.
pub fn option_number_of_option_string<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Format<T> {
        String(String),
        Number(T),
    }

    match Option::<Format<T>>::deserialize(deserializer) {
        Ok(res) => match res {
            Some(op) => Ok(match op {
                Format::String(s) => s.parse::<T>().map_or(None, Some),
                Format::Number(i) => Some(i),
            }),
            None => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

/// Deserialize possible `Option<Vec<String>>` into `Option<Vec<T>>`.
pub fn option_number_vec_of_option_string_vec<'de, T, D>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Format<T> {
        String(Vec<String>),
        Number(Vec<T>),
    }

    match Option::<Format<T>>::deserialize(deserializer) {
        Ok(res) => match res {
            Some(value) => Ok(match value {
                Format::String(v) => v
                    .iter()
                    .map(|s| s.parse::<T>().map_or(None, Some))
                    .collect(),
                Format::Number(i) => Some(i),
            }),
            None => Ok(None),
        },
        Err(_) => Ok(None),
    }
}
