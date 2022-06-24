use tezos_core::{
    internal::types::BytesTag,
    types::encoded::{Address, ChainId, Encoded, Key, Signature},
};

use crate::{
    micheline::{
        literals::{Bytes, Literal},
        prim_with_args,
        primitive_application::PrimitiveApplication,
        sequence::Sequence,
        Micheline,
    },
    michelson::{
        ComparableTypePrimitive, DataPrimitive, InstructionPrimitive, Primitive, TypePrimitive,
    },
    Error, Result,
};

pub trait Packer<T> {
    type Error;

    fn pack(value: T, schema: Option<&Micheline>) -> std::result::Result<Vec<u8>, Error>;
    fn unpack(bytes: &[u8], schema: Option<&Micheline>) -> std::result::Result<T, Error>;
}

pub struct MichelinePacker;

impl MichelinePacker {
    fn pre_pack(value: Micheline, schema: &Micheline) -> Result<Micheline> {
        match schema {
            Micheline::PrimitiveApplication(primitive_application) => {
                Self::pre_pack_primitive_application(value, primitive_application)
            }
            Micheline::Sequence(sequence) => {
                Self::pre_pack_sequence(value.try_into()?, sequence).map(|value| value.into())
            }
            _ => Err(Error::InvalidMicheline),
        }
    }

    fn pre_pack_primitive_application(
        value: Micheline,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let prim: Primitive = schema.prim().try_into()?;
        match prim {
            Primitive::Type(TypePrimitive::Option) => {
                Self::pre_pack_option(value.try_into()?, schema)
            }
            Primitive::Type(TypePrimitive::Or) => Self::pre_pack_or(value.try_into()?, schema),
            Primitive::Type(TypePrimitive::List) | Primitive::Type(TypePrimitive::Set) => {
                Self::pre_pack_data_sequence(value.try_into()?, schema)
            }
            Primitive::Type(TypePrimitive::Contract)
            | Primitive::ComparableType(ComparableTypePrimitive::Address) => {
                Self::pre_pack_address(value.try_into()?)
            }
            Primitive::Type(TypePrimitive::Pair) => Self::pre_pack_pair(value, schema),
            Primitive::Type(TypePrimitive::Lambda) => {
                Self::pre_pack_lambda(value.try_into()?, schema)
            }
            Primitive::Type(TypePrimitive::Map) => Self::pre_pack_map(value.try_into()?, schema),
            Primitive::Type(TypePrimitive::BigMap) => Self::pre_pack_big_map(value, schema),
            Primitive::ComparableType(ComparableTypePrimitive::ChainId) => {
                Self::pre_pack_chain_id(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::KeyHash) => {
                Self::pre_pack_key_hash(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::Key) => {
                Self::pre_pack_key(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::Signature) => {
                Self::pre_pack_signature(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::Timestamp) => {
                Self::pre_pack_timestamp(value.try_into()?)
            }
            _ => Ok(value),
        }
    }

    fn pre_pack_sequence(value: Sequence, schema: &Sequence) -> Result<Sequence> {
        if value.values().len() != schema.values().len() {
            return Err(Error::MichelineValueSchemaMismatch);
        }
        let values = Self::pre_pack_values(value.into_values(), schema.values())?;
        Ok(values.into())
    }

    fn pre_pack_values(values: Vec<Micheline>, schemas: &[Micheline]) -> Result<Vec<Micheline>> {
        values
            .into_iter()
            .zip(schemas)
            .map(|(value, schema)| Self::pre_pack(value, schema))
            .collect::<Result<Vec<_>>>()
    }

    fn pre_pack_vec(values: Vec<Micheline>, schema: &Micheline) -> Result<Vec<Micheline>> {
        values
            .into_iter()
            .map(|value| Self::pre_pack(value, schema))
            .collect::<Result<Vec<_>>>()
    }

    fn pre_pack_option(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let primitive = value.prim().parse::<DataPrimitive>()?;
        match (primitive, value.args(), schema.args()) {
            (DataPrimitive::Some, Some(values), Some(schemas)) if values.len() == schemas.len() => {
                value
                    .try_with_mutated_args(|values| Self::pre_pack_values(values, schemas))
                    .map(|value| value.into())
            }
            (DataPrimitive::None, _, _) => Ok(value.into()),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn pre_pack_or(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() != 1 && schema.args_count() != 2 {
            return Err(Error::MichelineValueSchemaMismatch);
        }
        let primitive = value.prim().parse::<DataPrimitive>()?;
        let value_type = match (primitive, value.args(), schema.args()) {
            (DataPrimitive::Left, Some(_), Some(schemas)) => Ok(schemas.first().unwrap()),
            (DataPrimitive::Right, Some(_), Some(schemas)) => Ok(schemas.iter().nth(1).unwrap()),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }?;
        Ok(value
            .try_with_mutated_args(|values| Self::pre_pack_vec(values, value_type))?
            .into())
    }

    fn pre_pack_data_sequence(value: Sequence, schema: &PrimitiveApplication) -> Result<Micheline> {
        if schema.args_count() != 1 {
            return Err(Error::MichelineValueSchemaMismatch);
        }
        Self::pre_pack_vec(value.into_values(), schema.first_arg().unwrap())
            .map(|values| values.into())
    }

    fn pre_pack_address(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<Address>(value)
    }

    fn pre_pack_encoded<E: Encoded>(value: Literal) -> Result<Micheline>
    where
        E: TryFrom<String>,
        Error: From<<E as TryFrom<String>>::Error>,
    {
        match value {
            Literal::String(value) => {
                let encoded: E = value.into_string().try_into()?;
                let bytes: Bytes = encoded.to_bytes()?.into();
                Ok(bytes.into())
            }
            Literal::Bytes(_) => Ok(value.into()),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn pre_pack_pair(value: Micheline, schema: &PrimitiveApplication) -> Result<Micheline> {
        if value.is_micheline_sequence() {
            let args = value.into_micheline_sequence().unwrap().into_values();
            let pair = prim_with_args(Primitive::Data(DataPrimitive::Pair), args).normalized();
            return Self::pre_pack_pair(pair, schema);
        }
        let value = value
            .into_micheline_primitive_application()
            .ok_or(Error::MichelineValueSchemaMismatch)?;
        let primitive = value.prim().parse::<DataPrimitive>()?;
        if let DataPrimitive::Pair = primitive {
            let value = value.normalized();
            let schema = schema.clone().normalized();
            if value.args_count() != schema.args_count() {
                return Err(Error::MichelineValueSchemaMismatch);
            }
            return value
                .try_with_mutated_args(|args| {
                    Self::pre_pack_values(args, schema.args().as_ref().unwrap())
                })
                .map(|value| value.into());
        }
        Err(Error::MichelineValueSchemaMismatch)
    }

    fn pre_pack_lambda(value: Sequence, schema: &PrimitiveApplication) -> Result<Micheline> {
        let values = value.into_values();
        Ok(values
            .into_iter()
            .map(|value| match value {
                Micheline::Literal(_) => Err(Error::MichelineValueSchemaMismatch),
                Micheline::PrimitiveApplication(value) => Self::pre_pack_instruction(value, schema),
                Micheline::Sequence(value) => Self::pre_pack_lambda(value, schema),
            })
            .collect::<Result<Vec<_>>>()?
            .into())
    }

    fn pre_pack_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let primitive = value.prim().parse::<InstructionPrimitive>()?;
        match primitive {
            InstructionPrimitive::Map | InstructionPrimitive::Iter => {
                Self::pre_pack_iter_instruction(value, schema)
            }
            InstructionPrimitive::Loop | InstructionPrimitive::LoopLeft => {
                Self::pre_pack_loop_instruction(value, schema)
            }
            InstructionPrimitive::Lambda => Self::pre_pack_lambda_instruction(value, schema),
            InstructionPrimitive::Dip => Self::pre_pack_dip_instruction(value, schema),
            InstructionPrimitive::IfNone
            | InstructionPrimitive::IfLeft
            | InstructionPrimitive::IfCons
            | InstructionPrimitive::If => Self::pre_pack_if_instruction(value, schema),
            InstructionPrimitive::Push => Self::pre_pack_push_instruction(value),
            InstructionPrimitive::CreateContract => {
                Self::pre_pack_create_contract_instruction(value, schema)
            }
            _ => Ok(value.into()),
        }
    }

    fn pre_pack_iter_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() == 0 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(0, |value| Self::pre_pack_lambda(value.try_into()?, schema))?
            .into())
    }

    fn pre_pack_loop_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() == 0 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(0, |value| Self::pre_pack_lambda(value.try_into()?, schema))?
            .into())
    }

    fn pre_pack_lambda_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() < 3 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(2, |value| Self::pre_pack_lambda(value.try_into()?, schema))?
            .into())
    }

    fn pre_pack_dip_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let count = value.args_count();
        if count == 0 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(count - 1, |value| {
                Self::pre_pack_lambda(value.try_into()?, schema)
            })?
            .into())
    }

    fn pre_pack_create_contract_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() < 3 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(2, |value| Self::pre_pack_lambda(value.try_into()?, schema))?
            .into())
    }

    fn pre_pack_if_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() != 2 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_mutated_args(|values| {
                values
                    .into_iter()
                    .map(|value| Ok(Self::pre_pack_lambda(value.try_into()?, schema)?))
                    .collect::<Result<Vec<_>>>()
            })?
            .into())
    }

    fn pre_pack_push_instruction(value: PrimitiveApplication) -> Result<Micheline> {
        if value.args_count() != 2 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_mutated_args::<_, Error>(|values| {
                let mut values = values;
                let (schema, value) = (values.remove(0), values.remove(0));
                let value = Self::pre_pack(value, &schema)?;
                Ok(vec![schema, value])
            })?
            .into())
    }

    fn pre_pack_map(value: Sequence, schema: &PrimitiveApplication) -> Result<Micheline> {
        let values = value.into_values();
        Ok(values
            .into_iter()
            .map(|value| {
                let value = value
                    .into_micheline_primitive_application()
                    .ok_or(Error::MichelineValueSchemaMismatch)?;
                if value.args_count() != schema.args_count() {
                    return Err(Error::MichelineValueSchemaMismatch);
                }
                let primitive = value.prim().parse::<DataPrimitive>()?;
                if let DataPrimitive::Elt = primitive {
                    return Ok(value
                        .try_with_mutated_args(|values| {
                            Self::pre_pack_values(
                                values,
                                schema
                                    .args()
                                    .as_ref()
                                    .ok_or(Error::MichelineValueSchemaMismatch)?,
                            )
                        })?
                        .into());
                }
                Err(Error::MichelineValueSchemaMismatch)
            })
            .collect::<Result<Vec<_>>>()?
            .into())
    }

    fn pre_pack_big_map(value: Micheline, schema: &PrimitiveApplication) -> Result<Micheline> {
        match value {
            Micheline::Literal(Literal::Int(_)) => Ok(value),
            Micheline::Sequence(value) => Self::pre_pack_map(value, schema),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn pre_pack_chain_id(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<ChainId>(value)
    }

    fn pre_pack_key_hash(value: Literal) -> Result<Micheline> {
        todo!()
    }

    fn pre_pack_key(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<Key>(value)
    }

    fn pre_pack_signature(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<Signature>(value)
    }

    fn pre_pack_timestamp(value: Literal) -> Result<Micheline> {
        todo!()
    }
}

impl Packer<Micheline> for MichelinePacker {
    type Error = Error;

    fn pack(value: Micheline, schema: Option<&Micheline>) -> Result<Vec<u8>> {
        if let Some(schema) = schema {
            let pre_packed = Self::pre_pack(value, schema)?;
            Ok(Tag::Message.prefixed_to(&pre_packed.to_bytes()?))
        } else {
            Ok(Tag::Message.prefixed_to(&value.to_bytes()?))
        }
    }

    fn unpack(bytes: &[u8], schema: Option<&Micheline>) -> Result<Micheline> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum Tag {
    Message,
}

impl BytesTag for Tag {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::Message => &[5],
        }
    }
}
