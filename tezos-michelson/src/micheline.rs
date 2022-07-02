pub mod literals;
pub mod primitive_application;
pub mod sequence;
mod utils;

use serde::{Deserialize, Serialize};
use tezos_core::internal::{
    coder::{Decoder, Encoder},
    normalizer::Normalizer,
};

pub use self::utils::{
    bytes, int, primitive_application, sequence, string, try_bytes, try_int, try_string,
};
use self::{literals::Literal, primitive_application::PrimitiveApplication, sequence::Sequence};
use crate::{
    internal::{
        coder::micheline_bytes_coder::MichelineBytesCoder,
        normalizer::MichelineNormalizer,
        packer::{MichelinePacker, Packer},
    },
    michelson::{types::Type, Michelson},
    Error, Result,
};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum Micheline {
    Literal(Literal),
    PrimitiveApplication(PrimitiveApplication),
    Sequence(Sequence),
}

impl Micheline {
    pub fn pack(self, schema: Option<&Micheline>) -> Result<Vec<u8>> {
        MichelinePacker::pack(self, schema)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        MichelineBytesCoder::encode(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        MichelineBytesCoder::decode(bytes)
    }

    pub fn is_micheline_literal(&self) -> bool {
        if let Self::Literal(_) = self {
            return true;
        }
        return false;
    }

    pub fn is_micheline_primitive_application(&self) -> bool {
        if let Self::PrimitiveApplication(_) = self {
            return true;
        }
        return false;
    }

    pub fn is_micheline_sequence(&self) -> bool {
        if let Self::Sequence(_) = self {
            return true;
        }
        return false;
    }

    pub fn into_micheline_literal(self) -> Option<Literal> {
        if let Self::Literal(value) = self {
            return Some(value);
        }
        return None;
    }

    pub fn into_micheline_primitive_application(self) -> Option<PrimitiveApplication> {
        if let Self::PrimitiveApplication(value) = self {
            return Some(value);
        }
        return None;
    }

    pub fn into_micheline_sequence(self) -> Option<Sequence> {
        if let Self::Sequence(value) = self {
            return Some(value);
        }
        return None;
    }

    pub fn normalized(self) -> Self {
        MichelineNormalizer::normalize(self)
    }
}

impl From<Literal> for Micheline {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl From<PrimitiveApplication> for Micheline {
    fn from(value: PrimitiveApplication) -> Self {
        Self::PrimitiveApplication(value)
    }
}

impl From<Sequence> for Micheline {
    fn from(value: Sequence) -> Self {
        Self::Sequence(value)
    }
}

impl From<Vec<Micheline>> for Micheline {
    fn from(value: Vec<Micheline>) -> Self {
        Self::Sequence(value.into())
    }
}

impl TryFrom<&Micheline> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &Micheline) -> Result<Self> {
        value.to_bytes()
    }
}

impl TryFrom<&[u8]> for Micheline {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        Micheline::from_bytes(value)
    }
}

impl From<Michelson> for Micheline {
    fn from(value: Michelson) -> Self {
        match value {
            Michelson::Data(value) => value.into(),
            Michelson::Type(value) => match value {
                Type::Comparable(value) => value.into(),
                Type::Parameter(value) => value.into(),
                Type::Storage(value) => value.into(),
                Type::Code(value) => value.into(),
                Type::Option(value) => value.into(),
                Type::List(value) => value.into(),
                Type::Set(value) => value.into(),
                Type::Operation(value) => value.into(),
                Type::Contract(value) => value.into(),
                Type::Ticket(value) => value.into(),
                Type::Pair(value) => value.into(),
                Type::Or(value) => value.into(),
                Type::Lambda(value) => value.into(),
                Type::Map(value) => value.into(),
                Type::BigMap(value) => value.into(),
                Type::Bls12_381G1(value) => value.into(),
                Type::Bls12_381G2(value) => value.into(),
                Type::Bls12_381Fr(value) => value.into(),
                Type::SaplingTransaction(value) => value.into(),
                Type::SaplingState(value) => value.into(),
                Type::Chest(value) => value.into(),
                Type::ChestKey(value) => value.into(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_micheline_to_json() -> Result<()> {
        for (value, json) in micheline_values() {
            assert_eq!(json, json!(value));
        }
        Ok(())
    }

    #[test]
    fn test_json_to_micheline() -> Result<()> {
        for (value, json) in micheline_values() {
            assert_eq!(value, serde_json::from_value(json)?);
        }
        Ok(())
    }

    fn micheline_values() -> Vec<(Micheline, serde_json::Value)> {
        vec![
            (int(0), json!({"int": "0"})),
            (try_string("string").unwrap(), json!({"string": "string"})),
            (try_bytes("0x").unwrap(), json!({"bytes": ""})),
            (try_bytes("0x00").unwrap(), json!({"bytes": "00"})),
            (
                primitive_application("Unit").into(),
                json!({"prim": "Unit"}),
            ),
            (
                primitive_application("Dig").with_args(vec![int(0)]).into(),
                json!({"prim": "Dig", "args": [{"int": "0"}]}),
            ),
            (
                primitive_application("Unit")
                    .with_annots(vec!["%annot".into()])
                    .into(),
                json!({"prim": "Unit", "annots": ["%annot"]}),
            ),
            (
                primitive_application("Dig")
                    .with_args(vec![int(0)])
                    .with_annots(vec!["%annot".into()])
                    .into(),
                json!({"prim": "Dig", "args": [{"int": "0"}], "annots": ["%annot"]}),
            ),
            (vec![].into(), json!([])),
            (vec![int(0)].into(), json!([{"int": "0"}])),
            (
                vec![primitive_application("Dig")
                    .with_args(vec![int(0)])
                    .with_annots(vec!["%annot".into()])
                    .into()]
                .into(),
                json!([{"prim": "Dig", "args": [{"int": "0"}], "annots": ["%annot"]}]),
            ),
        ]
    }
}
