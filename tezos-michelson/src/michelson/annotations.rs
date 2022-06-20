use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Type,
    Variable,
    Field,
}

impl Kind {
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Type => ":",
            Self::Variable => "@",
            Self::Field => "%",
        }
    }

    fn values() -> &'static [Kind] {
        &[Self::Type, Self::Variable, Self::Field]
    }

    fn recognize(value: &str) -> Option<Self> {
        Self::values()
            .iter()
            .find(|kind| kind.is_valid(value))
            .map(|kind| *kind)
    }

    pub fn is_valid(&self, value: &str) -> bool {
        value.starts_with(self.prefix())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    kind: Kind,
    value: String,
}

impl Annotation {
    pub fn new(value: String) -> Result<Self> {
        let kind = Kind::recognize(&value).ok_or(Error::InvalidAnnotationString)?;
        Ok(Self { kind, value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn matches(&self, string: &str) -> bool {
        let prefix = self.kind.prefix();
        self.value.strip_prefix(prefix) == string.strip_prefix(prefix)
    }
}

impl TryFrom<String> for Annotation {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}
