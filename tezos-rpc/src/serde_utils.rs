use {
    serde::{Deserialize, Deserializer},
    std::str::FromStr,
};

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
