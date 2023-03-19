use tezos_core::internal::normalizer::Normalizer;

use crate::{
    micheline::{
        primitive_application, primitive_application::PrimitiveApplication, sequence::Sequence,
        Micheline,
    },
    michelson::{
        self,
        data::{self, instructions, Data, Instruction},
        types::{self, ComparableType, Type},
        Michelson, PrimType,
    },
};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

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
                    Self::normalize(primitive_application(prim).with_args(args).into()),
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

pub struct MichelsonNormalizer;

impl Normalizer<Michelson> for MichelsonNormalizer {
    fn normalize(value: Michelson) -> Michelson {
        match value {
            Michelson::Data(value) => Michelson::Data(Self::normalize(value)),
            Michelson::Type(value) => Michelson::Type(Self::normalize(value)),
        }
    }
}

impl Normalizer<Data> for MichelsonNormalizer {
    fn normalize(value: Data) -> Data {
        match value {
            Data::Sequence(value) => Data::Sequence(
                value
                    .into_values()
                    .into_iter()
                    .map(|value| Self::normalize(value))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            Data::Map(value) => Data::Map(
                value
                    .into_values()
                    .into_iter()
                    .map(|value| Self::normalize(value))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            Data::Instruction(value) => Data::Instruction(Self::normalize(value)),
            Data::Pair(value) => Data::Pair(Self::normalize(value)),
            Data::Left(value) => data::Left::new(Self::normalize(*value.value)).into(),
            Data::Right(value) => data::Right::new(Self::normalize(*value.value)).into(),
            Data::Some(value) => data::Some::new(Self::normalize(*value.value)).into(),
            Data::Elt(value) => Data::Elt(Self::normalize(value)),
            _ => value,
        }
    }
}

impl Normalizer<Type> for MichelsonNormalizer {
    fn normalize(value: Type) -> Type {
        match value {
            Type::Parameter(value) => {
                types::Parameter::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::Storage(value) => {
                types::Storage::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::Code(value) => {
                types::Code::new(Self::normalize(*value.code), Some(value.metadata)).into()
            }
            Type::Option(value) => {
                types::Option::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::List(value) => {
                types::List::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::Set(value) => {
                types::Set::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::Contract(value) => {
                types::Contract::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::Ticket(value) => {
                types::Ticket::new(Self::normalize(*value.r#type), Some(value.metadata)).into()
            }
            Type::Pair(value) => Type::Pair(Self::normalize(value)),
            Type::Or(value) => types::Or::new(
                Self::normalize(*value.lhs),
                Self::normalize(*value.rhs),
                Some(value.metadata),
            )
            .into(),
            Type::Lambda(value) => types::Lambda::new(
                Self::normalize(*value.parameter_type),
                Self::normalize(*value.return_type),
                Some(value.metadata),
            )
            .into(),
            Type::Map(value) => types::Map::new(
                Self::normalize(*value.key_type),
                Self::normalize(*value.value_type),
                Some(value.metadata),
            )
            .into(),
            Type::BigMap(value) => types::BigMap::new(
                Self::normalize(*value.key_type),
                Self::normalize(*value.value_type),
                Some(value.metadata),
            )
            .into(),
            _ => value,
        }
    }
}

impl Normalizer<ComparableType> for MichelsonNormalizer {
    fn normalize(value: ComparableType) -> ComparableType {
        match value {
            ComparableType::Option(value) => {
                types::ComparableOption::new(Self::normalize(*value.r#type), Some(value.metadata))
                    .into()
            }
            ComparableType::Or(value) => types::ComparableOr::new(
                Self::normalize(*value.lhs),
                Self::normalize(*value.rhs),
                Some(value.metadata),
            )
            .into(),
            ComparableType::Pair(value) => Self::normalize(value).into(),
            _ => value,
        }
    }
}

impl Normalizer<data::Pair> for MichelsonNormalizer {
    fn normalize(value: data::Pair) -> data::Pair {
        if value.values.len() > 2 {
            let mut values = value.values;
            let first = values.remove(0);
            data::Pair::new(vec![
                Self::normalize(first),
                Self::normalize(data::Pair::new(values).into()),
            ])
        } else {
            let values = value.values;
            data::Pair::new(
                values
                    .into_iter()
                    .map(|value| Self::normalize(value))
                    .collect(),
            )
        }
    }
}

impl Normalizer<types::Pair> for MichelsonNormalizer {
    fn normalize(value: types::Pair) -> types::Pair {
        if value.types.len() > 2 {
            let mut values = value.types;
            let first = values.remove(0);
            types::Pair::new(
                vec![
                    Self::normalize(first),
                    Self::normalize(types::Pair::new(values, None).into()),
                ],
                Some(value.metadata),
            )
        } else {
            let values = value.types;
            types::Pair::new(
                values
                    .into_iter()
                    .map(|value| Self::normalize(value))
                    .collect(),
                Some(value.metadata),
            )
        }
    }
}

impl Normalizer<types::ComparablePair> for MichelsonNormalizer {
    fn normalize(value: types::ComparablePair) -> types::ComparablePair {
        if value.types.len() > 2 {
            let mut values = value.types;
            let first = values.remove(0);
            types::ComparablePair::new(
                vec![
                    Self::normalize(first),
                    Self::normalize(types::ComparablePair::new(values, None).into()),
                ],
                Some(value.metadata),
            )
        } else {
            let values = value.types;
            types::ComparablePair::new(
                values
                    .into_iter()
                    .map(|value| Self::normalize(value))
                    .collect(),
                Some(value.metadata),
            )
        }
    }
}

impl Normalizer<data::Elt> for MichelsonNormalizer {
    fn normalize(value: data::Elt) -> data::Elt {
        data::Elt::new(Self::normalize(*value.key), Self::normalize(*value.value))
    }
}

impl Normalizer<Instruction> for MichelsonNormalizer {
    fn normalize(value: Instruction) -> Instruction {
        match value {
            Instruction::Sequence(value) => Instruction::Sequence(Self::normalize(value)),
            Instruction::Iter(value) => {
                instructions::Iter::new(Self::normalize(value.expression)).into()
            }
            Instruction::LoopLeft(value) => {
                instructions::LoopLeft::new(Self::normalize(value.body)).into()
            }
            Instruction::Loop(value) => instructions::Loop::new(Self::normalize(value.body)).into(),
            Instruction::Dip(value) => {
                instructions::Dip::new(value.n, Self::normalize(value.instruction)).into()
            }
            Instruction::EmptyBigMap(value) => instructions::EmptyBigMap::new(
                Self::normalize(value.key_type),
                Self::normalize(value.value_type),
                value.metadata,
            )
            .into(),
            Instruction::EmptyMap(value) => instructions::EmptyMap::new(
                Self::normalize(value.key_type),
                Self::normalize(value.value_type),
                value.metadata,
            )
            .into(),
            Instruction::EmptySet(value) => {
                instructions::EmptySet::new(Self::normalize(value.r#type), value.metadata).into()
            }
            Instruction::IfCons(value) => instructions::IfCons::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )
            .into(),
            Instruction::IfLeft(value) => instructions::IfLeft::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )
            .into(),
            Instruction::IfNone(value) => instructions::IfNone::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )
            .into(),
            Instruction::If(value) => instructions::If::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )
            .into(),
            Instruction::Lambda(value) => instructions::Lambda::new(
                Self::normalize(value.parameter_type),
                Self::normalize(value.return_type),
                Self::normalize(value.body),
                value.metadata,
            )
            .into(),
            Instruction::Left(value) => {
                instructions::Left::new(Self::normalize(value.r#type), value.metadata).into()
            }
            Instruction::Right(value) => {
                instructions::Right::new(Self::normalize(value.r#type), value.metadata).into()
            }
            Instruction::Map(value) => {
                instructions::Map::new(Self::normalize(value.expression), value.metadata).into()
            }
            Instruction::Nil(value) => {
                instructions::Nil::new(Self::normalize(value.r#type), value.metadata).into()
            }
            Instruction::None(value) => {
                instructions::None::new(Self::normalize(value.r#type), value.metadata).into()
            }
            Instruction::Push(value) => instructions::Push::new(
                Self::normalize(value.r#type),
                Self::normalize(*value.value),
                value.metadata,
            )
            .into(),
            Instruction::Contract(value) => {
                instructions::Contract::new(Self::normalize(value.r#type), value.metadata).into()
            }
            Instruction::CreateContract(value) => instructions::CreateContract::new(
                Self::normalize(value.parameter_type),
                Self::normalize(value.storage_type),
                Self::normalize(value.code),
                value.metadata,
            )
            .into(),
            _ => value,
        }
    }
}

impl Normalizer<instructions::Sequence> for MichelsonNormalizer {
    fn normalize(value: instructions::Sequence) -> instructions::Sequence {
        value
            .into_instructions()
            .into_iter()
            .map(|value| Self::normalize(value))
            .collect::<Vec<_>>()
            .into()
    }
}
