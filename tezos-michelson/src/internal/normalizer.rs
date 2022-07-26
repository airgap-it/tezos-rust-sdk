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
            Data::Left(value) => Data::Left(data::Left::new(Self::normalize(*value.value))),
            Data::Right(value) => Data::Right(data::Right::new(Self::normalize(*value.value))),
            Data::Some(value) => Data::Some(data::Some::new(Self::normalize(*value.value))),
            Data::Elt(value) => Data::Elt(Self::normalize(value)),
            _ => value,
        }
    }
}

impl Normalizer<Type> for MichelsonNormalizer {
    fn normalize(value: Type) -> Type {
        match value {
            Type::Parameter(value) => Type::Parameter(types::Parameter::new(
                Self::normalize(*value.r#type),
                Some(value.metadata),
            )),
            Type::Storage(value) => Type::Storage(types::Storage::new(
                Self::normalize(*value.r#type),
                Some(value.metadata),
            )),
            Type::Code(value) => Type::Code(types::Code::new(
                Self::normalize(*value.code),
                Some(value.metadata),
            )),
            Type::Option(value) => Type::Option(types::Option::new(
                Self::normalize(*value.r#type),
                Some(value.metadata),
            )),
            Type::List(value) => Type::List(types::List::new(
                Self::normalize(*value.r#type),
                Some(value.metadata),
            )),
            Type::Set(value) => Type::Set(types::Set::new(
                Self::normalize(value.r#type),
                Some(value.metadata),
            )),
            Type::Contract(value) => Type::Contract(types::Contract::new(
                Self::normalize(*value.r#type),
                Some(value.metadata),
            )),
            Type::Ticket(value) => Type::Ticket(types::Ticket::new(
                Self::normalize(*value.r#type),
                Some(value.metadata),
            )),
            Type::Pair(value) => Type::Pair(Self::normalize(value)),
            Type::Or(value) => Type::Or(types::Or::new(
                Self::normalize(*value.lhs),
                Self::normalize(*value.rhs),
                Some(value.metadata),
            )),
            Type::Lambda(value) => Type::Lambda(types::Lambda::new(
                Self::normalize(*value.parameter_type),
                Self::normalize(*value.return_type),
                Some(value.metadata),
            )),
            Type::Map(value) => Type::Map(types::Map::new(
                Self::normalize(*value.key_type),
                Self::normalize(*value.value_type),
                Some(value.metadata),
            )),
            Type::BigMap(value) => Type::BigMap(types::BigMap::new(
                Self::normalize(*value.key_type),
                Self::normalize(*value.value_type),
                Some(value.metadata),
            )),
            _ => value,
        }
    }
}

impl Normalizer<ComparableType> for MichelsonNormalizer {
    fn normalize(value: ComparableType) -> ComparableType {
        match value {
            ComparableType::Option(_) => todo!(),
            ComparableType::Or(_) => todo!(),
            ComparableType::Pair(_) => todo!(),
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
                Instruction::Iter(instructions::Iter::new(Self::normalize(value.expression)))
            }
            Instruction::LoopLeft(value) => {
                Instruction::LoopLeft(instructions::LoopLeft::new(Self::normalize(value.body)))
            }
            Instruction::Loop(value) => {
                Instruction::Loop(instructions::Loop::new(Self::normalize(value.body)))
            }
            Instruction::Dip(value) => Instruction::Dip(instructions::Dip::new(
                Self::normalize(value.instruction),
                value.n,
            )),
            Instruction::EmptyBigMap(value) => {
                Instruction::EmptyBigMap(instructions::EmptyBigMap::new(
                    Self::normalize(value.key_type),
                    Self::normalize(value.value_type),
                    value.metadata,
                ))
            }
            Instruction::EmptyMap(value) => Instruction::EmptyMap(instructions::EmptyMap::new(
                Self::normalize(value.key_type),
                Self::normalize(value.value_type),
                value.metadata,
            )),
            Instruction::EmptySet(value) => Instruction::EmptySet(instructions::EmptySet::new(
                Self::normalize(value.r#type),
                value.metadata,
            )),
            Instruction::IfCons(value) => Instruction::IfCons(instructions::IfCons::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )),
            Instruction::IfLeft(value) => Instruction::IfLeft(instructions::IfLeft::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )),
            Instruction::IfNone(value) => Instruction::IfNone(instructions::IfNone::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )),
            Instruction::If(value) => Instruction::If(instructions::If::new(
                Self::normalize(value.if_branch),
                Self::normalize(value.else_branch),
            )),
            Instruction::Lambda(value) => Instruction::Lambda(instructions::Lambda::new(
                Self::normalize(value.parameter_type),
                Self::normalize(value.return_type),
                Self::normalize(value.body),
                value.metadata,
            )),
            Instruction::Left(value) => Instruction::Left(instructions::Left::new(
                Self::normalize(value.r#type),
                value.metadata,
            )),
            Instruction::Right(value) => Instruction::Right(instructions::Right::new(
                Self::normalize(value.r#type),
                value.metadata,
            )),
            Instruction::Map(value) => Instruction::Map(instructions::Map::new(
                Self::normalize(value.expression),
                value.metadata,
            )),
            Instruction::Nil(value) => Instruction::Nil(instructions::Nil::new(
                Self::normalize(value.r#type),
                value.metadata,
            )),
            Instruction::None(value) => Instruction::None(instructions::None::new(
                Self::normalize(value.r#type),
                value.metadata,
            )),
            Instruction::Push(value) => Instruction::Push(instructions::Push::new(
                Self::normalize(value.r#type),
                Self::normalize(*value.value),
                value.metadata,
            )),
            Instruction::Contract(value) => Instruction::Contract(instructions::Contract::new(
                Self::normalize(value.r#type),
                value.metadata,
            )),
            Instruction::CreateContract(value) => {
                Instruction::CreateContract(instructions::CreateContract::new(
                    Self::normalize(value.parameter_type),
                    Self::normalize(value.storage_type),
                    Self::normalize(value.code),
                    value.metadata,
                ))
            }
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
