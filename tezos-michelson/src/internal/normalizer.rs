use tezos_core::internal::normalizer::Normalizer;

use crate::{
    micheline::{
        prim_with_args, primitive_application::PrimitiveApplication, sequence::Sequence, Micheline,
    },
    michelson::{self, PrimType},
};

pub struct MichelineNormalizer;

impl Normalizer<Micheline> for MichelineNormalizer {
    fn normalize(value: Micheline) -> Micheline {
        match value {
            Micheline::Literal(_) => value,
            Micheline::PrimitiveApplication(value) => Self::normalize(value).into(),
            Micheline::Sequence(value) => Self::normalize(value).into(),
        }
    }
}

impl Normalizer<PrimitiveApplication> for MichelineNormalizer {
    fn normalize(value: PrimitiveApplication) -> PrimitiveApplication {
        let pair_prims = [
            michelson::data::Pair::prim_value().name(),
            michelson::data::instructions::Pair::prim_value().name(),
            michelson::types::Pair::prim_value().name(),
        ];
        let args_len = value.args().as_ref().map(|args| args.len()).unwrap_or(0);
        if pair_prims.contains(&&value.prim()) && args_len > 2 {
            let prim: String = value.prim().into();
            value.with_mutated_args(|args| {
                let mut args = args;
                vec![
                    Self::normalize(args.remove(0)),
                    Self::normalize(prim_with_args(prim, args)),
                ]
            })
        } else {
            value.with_mutated_args(|args| {
                args.into_iter()
                    .map(|arg| Self::normalize(arg))
                    .collect::<Vec<_>>()
            })
        }
    }
}

impl Normalizer<Sequence> for MichelineNormalizer {
    fn normalize(value: Sequence) -> Sequence {
        value
            .into_values()
            .into_iter()
            .map(|value| Self::normalize(value))
            .collect::<Vec<_>>()
            .into()
    }
}
