use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use num_traits::ToPrimitive;
use tezos_core::{
    internal::types::BytesTag,
    types::encoded::{Address, ChainId, Encoded, ImplicitAddress, Key, Signature},
};

use crate::{
    micheline::{
        literals::{Bytes, Literal},
        primitive_application,
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
    pub fn pre_pack(value: Micheline, schema: &Micheline) -> Result<Micheline> {
        match schema {
            Micheline::PrimitiveApplication(primitive_application) => {
                Self::pre_pack_primitive_application(value, primitive_application)
            }
            Micheline::Sequence(sequence) => {
                Self::pre_pack_sequence(value.try_into()?, sequence).map(|value| value.into())
            }
            _ => Err(Error::InvalidMicheline { description: format!("Pre pack failed because provided schema ({:?}) is not a primitive application or sequence", schema) }),
        }
    }

    pub fn post_unpack(value: Micheline, schema: &Micheline) -> Result<Micheline> {
        match schema {
            Micheline::PrimitiveApplication(primitive_application) => {
                Self::post_unpack_primitive_application(value, primitive_application)
            }
            Micheline::Sequence(sequence) => {
                Self::post_unpack_sequence(value.try_into()?, sequence).map(|value| value.into())
            }
            _ => Err(Error::InvalidMicheline { description: format!("Post unpack failed because provided schema ({:?}) is not a primitive application or sequence", schema) }),
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

    fn post_unpack_primitive_application(
        value: Micheline,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let prim: Primitive = schema.prim().try_into()?;
        match prim {
            Primitive::Type(TypePrimitive::Option) => {
                Self::post_unpack_option(value.try_into()?, schema)
            }
            Primitive::Type(TypePrimitive::Or) => Self::post_unpack_or(value.try_into()?, schema),
            Primitive::Type(TypePrimitive::List) | Primitive::Type(TypePrimitive::Set) => {
                Self::post_unpack_data_sequence(value.try_into()?, schema)
            }
            Primitive::Type(TypePrimitive::Contract)
            | Primitive::ComparableType(ComparableTypePrimitive::Address) => {
                Self::post_unpack_address(value.try_into()?)
            }
            Primitive::Type(TypePrimitive::Pair) => Self::post_unpack_pair(value, schema),
            Primitive::Type(TypePrimitive::Lambda) => {
                Self::post_unpack_lambda(value.try_into()?, schema)
            }
            Primitive::Type(TypePrimitive::Map) => Self::post_unpack_map(value.try_into()?, schema),
            Primitive::Type(TypePrimitive::BigMap) => Self::post_unpack_big_map(value, schema),
            Primitive::ComparableType(ComparableTypePrimitive::ChainId) => {
                Self::post_unpack_chain_id(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::KeyHash) => {
                Self::post_unpack_key_hash(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::Key) => {
                Self::post_unpack_key(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::Signature) => {
                Self::post_unpack_signature(value.try_into()?)
            }
            Primitive::ComparableType(ComparableTypePrimitive::Timestamp) => {
                Self::post_unpack_timestamp(value.try_into()?)
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

    fn post_unpack_sequence(value: Sequence, schema: &Sequence) -> Result<Sequence> {
        if value.values().len() != schema.values().len() {
            return Err(Error::MichelineValueSchemaMismatch);
        }
        let values = Self::post_unpack_values(value.into_values(), schema.values())?;
        Ok(values.into())
    }

    fn pre_pack_values(values: Vec<Micheline>, schemas: &[Micheline]) -> Result<Vec<Micheline>> {
        values
            .into_iter()
            .zip(schemas)
            .map(|(value, schema)| Self::pre_pack(value, schema))
            .collect::<Result<Vec<_>>>()
    }

    fn post_unpack_values(values: Vec<Micheline>, schemas: &[Micheline]) -> Result<Vec<Micheline>> {
        values
            .into_iter()
            .zip(schemas)
            .map(|(value, schema)| Self::post_unpack(value, schema))
            .collect::<Result<Vec<_>>>()
    }

    fn pre_pack_vec(values: Vec<Micheline>, schema: &Micheline) -> Result<Vec<Micheline>> {
        values
            .into_iter()
            .map(|value| Self::pre_pack(value, schema))
            .collect::<Result<Vec<_>>>()
    }

    fn post_unpack_vec(values: Vec<Micheline>, schema: &Micheline) -> Result<Vec<Micheline>> {
        values
            .into_iter()
            .map(|value| Self::post_unpack(value, schema))
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

    fn post_unpack_option(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let primitive = value.prim().parse::<DataPrimitive>()?;
        match (primitive, value.args(), schema.args()) {
            (DataPrimitive::Some, Some(values), Some(schemas)) if values.len() == schemas.len() => {
                value
                    .try_with_mutated_args(|values| Self::post_unpack_values(values, schemas))
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

    fn post_unpack_or(
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
            .try_with_mutated_args(|values| Self::post_unpack_vec(values, value_type))?
            .into())
    }

    fn pre_pack_data_sequence(value: Sequence, schema: &PrimitiveApplication) -> Result<Micheline> {
        if schema.args_count() != 1 {
            return Err(Error::MichelineValueSchemaMismatch);
        }
        Self::pre_pack_vec(value.into_values(), schema.first_arg().unwrap())
            .map(|values| values.into())
    }

    fn post_unpack_data_sequence(
        value: Sequence,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if schema.args_count() != 1 {
            return Err(Error::MichelineValueSchemaMismatch);
        }
        Self::post_unpack_vec(value.into_values(), schema.first_arg().unwrap())
            .map(|values| values.into())
    }

    fn pre_pack_address(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<Address>(value)
    }

    fn post_unpack_address(value: Literal) -> Result<Micheline> {
        Self::post_unpack_encoded::<Address>(value)
    }

    fn pre_pack_encoded<E: Encoded>(value: Literal) -> Result<Micheline> {
        match value {
            Literal::String(value) => {
                let encoded: E = E::new(value.into_string())?;
                let bytes: Bytes = encoded.to_bytes()?.into();
                Ok(bytes.into())
            }
            Literal::Bytes(_) => Ok(value.into()),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn post_unpack_encoded<E: Encoded>(value: Literal) -> Result<Micheline> {
        match value {
            Literal::String(_) => Ok(value.into()),
            Literal::Bytes(value) => {
                let bytes: Vec<u8> = (&value).into();
                let encoded: E = E::from_bytes(&bytes)?;
                Ok(Literal::String(encoded.into_string().try_into()?).into())
            }
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn pre_pack_pair(value: Micheline, schema: &PrimitiveApplication) -> Result<Micheline> {
        if value.is_sequence() {
            let args = value.into_sequence().unwrap().into_values();
            let pair = primitive_application(DataPrimitive::Pair)
                .with_args(args)
                .normalized();
            return Self::pre_pack_pair(pair.into(), schema);
        }
        let value = value
            .into_primitive_application()
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

    fn post_unpack_pair(value: Micheline, schema: &PrimitiveApplication) -> Result<Micheline> {
        let value = value
            .into_primitive_application()
            .ok_or(Error::MichelineValueSchemaMismatch)?;
        let primitive = value.prim().parse::<DataPrimitive>()?;
        if let DataPrimitive::Pair = primitive {
            let schema = schema.clone().normalized();
            if value.args_count() != schema.args_count() {
                return Err(Error::MichelineValueSchemaMismatch);
            }
            return value
                .try_with_mutated_args(|args| {
                    Self::post_unpack_values(args, schema.args().as_ref().unwrap())
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

    fn post_unpack_lambda(value: Sequence, schema: &PrimitiveApplication) -> Result<Micheline> {
        let values = value.into_values();
        Ok(values
            .into_iter()
            .map(|value| match value {
                Micheline::Literal(_) => Err(Error::MichelineValueSchemaMismatch),
                Micheline::PrimitiveApplication(value) => {
                    Self::post_unpack_instruction(value, schema)
                }
                Micheline::Sequence(value) => Self::post_unpack_lambda(value, schema),
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

    fn post_unpack_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let primitive = value.prim().parse::<InstructionPrimitive>()?;
        match primitive {
            InstructionPrimitive::Map | InstructionPrimitive::Iter => {
                Self::post_unpack_iter_instruction(value, schema)
            }
            InstructionPrimitive::Loop | InstructionPrimitive::LoopLeft => {
                Self::post_unpack_loop_instruction(value, schema)
            }
            InstructionPrimitive::Lambda => Self::post_unpack_lambda_instruction(value, schema),
            InstructionPrimitive::Dip => Self::post_unpack_dip_instruction(value, schema),
            InstructionPrimitive::IfNone
            | InstructionPrimitive::IfLeft
            | InstructionPrimitive::IfCons
            | InstructionPrimitive::If => Self::post_unpack_if_instruction(value, schema),
            InstructionPrimitive::Push => Self::post_unpack_push_instruction(value),
            InstructionPrimitive::CreateContract => {
                Self::post_unpack_create_contract_instruction(value, schema)
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

    fn post_unpack_iter_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() == 0 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(0, |value| {
                Self::post_unpack_lambda(value.try_into()?, schema)
            })?
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

    fn post_unpack_loop_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() == 0 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(0, |value| {
                Self::post_unpack_lambda(value.try_into()?, schema)
            })?
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

    fn post_unpack_lambda_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() < 3 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(2, |value| {
                Self::post_unpack_lambda(value.try_into()?, schema)
            })?
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

    fn post_unpack_dip_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        let count = value.args_count();
        if count == 0 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(count - 1, |value| {
                Self::post_unpack_lambda(value.try_into()?, schema)
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

    fn post_unpack_create_contract_instruction(
        value: PrimitiveApplication,
        schema: &PrimitiveApplication,
    ) -> Result<Micheline> {
        if value.args_count() < 3 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_replaced_arg_at(2, |value| {
                Self::post_unpack_lambda(value.try_into()?, schema)
            })?
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

    fn post_unpack_if_instruction(
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
                    .map(|value| Ok(Self::post_unpack_lambda(value.try_into()?, schema)?))
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

    fn post_unpack_push_instruction(value: PrimitiveApplication) -> Result<Micheline> {
        if value.args_count() != 2 {
            return Err(Error::InvalidPrimitiveApplication);
        }
        Ok(value
            .try_with_mutated_args::<_, Error>(|values| {
                let mut values = values;
                let (schema, value) = (values.remove(0), values.remove(0));
                let value = Self::post_unpack(value, &schema)?;
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
                    .into_primitive_application()
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

    fn post_unpack_map(value: Sequence, schema: &PrimitiveApplication) -> Result<Micheline> {
        let values = value.into_values();
        Ok(values
            .into_iter()
            .map(|value| {
                let value = value
                    .into_primitive_application()
                    .ok_or(Error::MichelineValueSchemaMismatch)?;
                if value.args_count() != schema.args_count() {
                    return Err(Error::MichelineValueSchemaMismatch);
                }
                let primitive = value.prim().parse::<DataPrimitive>()?;
                if let DataPrimitive::Elt = primitive {
                    return Ok(value
                        .try_with_mutated_args(|values| {
                            Self::post_unpack_values(
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

    fn post_unpack_big_map(value: Micheline, schema: &PrimitiveApplication) -> Result<Micheline> {
        match value {
            Micheline::Literal(Literal::Int(_)) => Ok(value),
            Micheline::Sequence(value) => Self::post_unpack_map(value, schema),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn pre_pack_chain_id(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<ChainId>(value)
    }

    fn post_unpack_chain_id(value: Literal) -> Result<Micheline> {
        Self::post_unpack_encoded::<ChainId>(value)
    }

    fn pre_pack_key_hash(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<ImplicitAddress>(value)
    }

    fn post_unpack_key_hash(value: Literal) -> Result<Micheline> {
        Self::post_unpack_encoded::<ImplicitAddress>(value)
    }

    fn pre_pack_key(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<Key>(value)
    }

    fn post_unpack_key(value: Literal) -> Result<Micheline> {
        Self::post_unpack_encoded::<Key>(value)
    }

    fn pre_pack_signature(value: Literal) -> Result<Micheline> {
        Self::pre_pack_encoded::<Signature>(value)
    }

    fn post_unpack_signature(value: Literal) -> Result<Micheline> {
        Self::post_unpack_encoded::<Signature>(value)
    }

    fn pre_pack_timestamp(value: Literal) -> Result<Micheline> {
        match value {
            Literal::Int(_) => Ok(value.into()),
            Literal::String(value) => {
                let date_time = DateTime::parse_from_rfc3339(value.to_str())
                    .map_err(|_error| Error::MichelineValueSchemaMismatch)?;
                Ok(Literal::Int(date_time.timestamp_millis().into()).into())
            }
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
    }

    fn post_unpack_timestamp(value: Literal) -> Result<Micheline> {
        match value {
            Literal::Int(value) => {
                let value = value.to_i64().ok_or(Error::MichelineValueSchemaMismatch)?;
                let ts_secs = value / 1000;
                let ts_ns = (value % 1000) * 1_000_000;
                let dt = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(ts_secs, ts_ns as u32)
                        .expect("out-of-range number of seconds and/or invalid nanosecond"),
                    Utc,
                );
                Ok(
                    Literal::String(dt.to_rfc3339_opts(SecondsFormat::Millis, true).try_into()?)
                        .into(),
                )
            }
            Literal::String(_) => Ok(value.into()),
            _ => Err(Error::MichelineValueSchemaMismatch),
        }
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
        let tag = Tag::recognize(bytes).ok_or(Error::InvalidBytes)?;
        match tag {
            Tag::Message => {
                let value: Micheline = bytes[1..].try_into()?;
                if let Some(schema) = schema {
                    return Self::post_unpack(value, schema);
                }
                Ok(value)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tag {
    Message,
}

impl Tag {
    fn values() -> &'static [Self] {
        &[Self::Message]
    }

    fn recognize(bytes: &[u8]) -> Option<Self> {
        Self::values()
            .iter()
            .find(|item| bytes.starts_with(item.value()))
            .map(|item| *item)
    }
}

impl BytesTag for Tag {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::Message => &[5],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::michelson::{data, types};
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_pack() -> Result<()> {
        let tests_values = [
            integer_values(),
            string_values(),
            bytes_values(),
            primitive_application_values(),
            sequence_values(),
            vec![(
                &hex!("050707030b0707030b030b"),
                vec![data::unit(), data::unit(), data::unit()].into(),
                Some(types::pair(vec![
                    types::unit(),
                    types::unit(),
                    types::unit(),
                ])),
            )],
        ]
        .concat();
        for (bytes, value, schema) in tests_values {
            let packed = MichelinePacker::pack(value, schema.as_ref())?;
            assert_eq!(bytes, packed);
        }

        Ok(())
    }

    #[test]
    fn test_unpack() -> Result<()> {
        let tests_values = [
            integer_values(),
            string_values(),
            bytes_values(),
            primitive_application_values(),
            sequence_values(),
        ]
        .concat();
        for (bytes, value, schema) in tests_values {
            let unpacked = MichelinePacker::unpack(bytes, schema.as_ref())?;
            assert_eq!(value.normalized(), unpacked);
        }

        Ok(())
    }

    fn integer_values() -> Vec<(&'static [u8], Micheline, Option<Micheline>)> {
        vec![
            (
                &hex!("0500c384efcfc7dac2f5849995afab9fa7c48b8fa4c0d9b5ca908dc70d"),
                data::try_int("-41547452475632687683489977342365486797893454355756867843").unwrap(),
                Some(types::int()),
            ),
            (
                &hex!("0500fc90d3c2e6a3b9c4c0b7fbf3b3b6d802"),
                data::try_int("-54576326575686358562454576456764").unwrap(),
                Some(types::int()),
            ),
            (
                &hex!("0500c8a8dd9df89cb998be01"),
                data::int(-6852352674543413768i64),
                Some(types::int()),
            ),
            (
                &hex!("0500f9b1e2fee2c308"),
                data::int(-18756523543673i64),
                Some(types::int()),
            ),
            (&hex!("0500c002"), data::int(-128), Some(types::int())),
            (&hex!("0500ff01"), data::int(-127), Some(types::int())),
            (&hex!("0500c001"), data::int(-64), Some(types::int())),
            (&hex!("05006a"), data::int(-42), Some(types::int())),
            (&hex!("05004a"), data::int(-10), Some(types::int())),
            (&hex!("050041"), data::int(-1), Some(types::int())),
            (&hex!("050000"), data::int(0), Some(types::int())),
            (&hex!("050001"), data::int(1), Some(types::int())),
            (&hex!("05000a"), data::int(10), Some(types::int())),
            (&hex!("05002a"), data::int(42), Some(types::int())),
            (&hex!("05008001"), data::int(64), Some(types::int())),
            (&hex!("0500bf01"), data::int(127), Some(types::int())),
            (&hex!("05008002"), data::int(128), Some(types::int())),
            (
                &hex!("0500b9b1e2fee2c308"),
                data::int(18756523543673i64),
                Some(types::int()),
            ),
            (
                &hex!("050088a8dd9df89cb998be01"),
                data::int(6852352674543413768i64),
                Some(types::int()),
            ),
            (
                &hex!("0500bc90d3c2e6a3b9c4c0b7fbf3b3b6d802"),
                data::int(54576326575686358562454576456764i128),
                Some(types::int()),
            ),
            (
                &hex!("05008384efcfc7dac2f5849995afab9fa7c48b8fa4c0d9b5ca908dc70d"),
                data::try_int("41547452475632687683489977342365486797893454355756867843").unwrap(),
                Some(types::int()),
            ),
            (
                &hex!("05002a"),
                data::int(42),
                Some(types::big_map(
                    types::unit::<types::ComparableType>().into(),
                    types::unit::<types::ComparableType>().into(),
                )),
            ),
        ]
    }

    fn string_values() -> Vec<(&'static [u8], Micheline, Option<Micheline>)> {
        vec![
            (
                &hex!("050100000000"),
                data::try_string("").unwrap(),
                Some(types::string()),
            ),
            (
                &hex!("05010000000161"),
                data::try_string("a").unwrap(),
                Some(types::string()),
            ),
            (
                &hex!("050100000003616263"),
                data::try_string("abc").unwrap(),
                Some(types::string()),
            ),
            (
                &hex!("050100000024747a315a734b4d6f6f47504a6135486f525435445156356a31526b5263536979706e594e"),
                data::try_string("tz1ZsKMooGPJa5HoRT5DQV5j1RkRcSiypnYN").unwrap(),
                Some(types::string()),
            ),
            (
                &hex!("050a00000016000094a0ba27169ed8d97c1f476de6156c2482dbfb3d"),
                data::try_string("tz1ZBuF2dQ7E1b32bK3g1Qsah4pvWqpM4b4A").unwrap(),
                Some(types::address()),
            ),
            (
                &hex!("050a00000004ef6a66af"),
                data::try_string("NetXy3eo3jtuwuc").unwrap(),
                Some(types::chain_id()),
            ),
            (
                &hex!("050a000000150094a0ba27169ed8d97c1f476de6156c2482dbfb3d"),
                data::try_string("tz1ZBuF2dQ7E1b32bK3g1Qsah4pvWqpM4b4A").unwrap(),
                Some(types::key_hash()),
            ),
            (
                &hex!("050a00000021005a9847101250e9cea9e714a8fd945e5131aeb5c021e027b1420db0cdd971c862"),
                data::try_string("edpkuL84TEk6s2C9JCywmBS4Mztumq6iUVxNtBHvuZG95VPvFw1yCR").unwrap(),
                Some(types::key()),
            ),
            (
                &hex!("0500aff8aff1ce5f"),
                data::try_string("2022-01-20T10:43:57.103Z").unwrap(),
                Some(types::timestamp()),
            ),
            (
                &hex!("050a0000001a016077cd98fd8aca94851b83a4c44203b705d2004b006d696e74"),
                data::try_string("KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7%mint").unwrap(),
                Some(types::address())
            ),
        ]
    }

    fn bytes_values() -> Vec<(&'static [u8], Micheline, Option<Micheline>)> {
        vec![
            (
                &hex!("050a00000000"),
                data::try_bytes("0x").unwrap(),
                Some(types::bytes()),
            ),
            (
                &hex!("050a0000000100"),
                data::try_bytes("0x00").unwrap(),
                Some(types::bytes()),
            ),
            (
                &hex!("050a000000049434dc98"),
                data::try_bytes("0x9434dc98").unwrap(),
                Some(types::bytes()),
            ),
            (
                &hex!("050a000000047b1ea2cb"),
                data::try_bytes("0x7b1ea2cb").unwrap(),
                Some(types::bytes()),
            ),
            (
                &hex!("050a00000004e40476d7"),
                data::try_bytes("0xe40476d7").unwrap(),
                Some(types::bytes()),
            ),
            (
                &hex!("050a00000006c47320abdd31"),
                data::try_bytes("0xc47320abdd31").unwrap(),
                Some(types::bytes()),
            ),
            (
                &hex!("050a000000065786dac9eaf4"),
                data::try_bytes("0x5786dac9eaf4").unwrap(),
                Some(types::bytes()),
            ),
        ]
    }

    fn primitive_application_values() -> Vec<(&'static [u8], Micheline, Option<Micheline>)> {
        vec![
            (&hex!("050303"), data::r#false(), Some(types::bool())),
            (
                &hex!("050704030b030b"),
                data::elt(data::unit(), data::unit()),
                None,
            ),
            (
                &hex!("050505030b"),
                data::left(data::unit()),
                Some(types::or(types::unit(), types::bool())),
            ),
            (
                &hex!("0505050707030b0707030b030b"),
                data::left(data::pair(vec![data::unit(), data::unit(), data::unit()])),
                Some(types::or(
                    types::pair(vec![types::unit(), types::unit(), types::unit()]),
                    types::bool(),
                )),
            ),
            (
                &hex!("050306"),
                data::none(),
                Some(types::option(types::pair(vec![
                    types::unit(),
                    types::unit(),
                    types::unit(),
                ]))),
            ),
            (&hex!("050303"), data::r#false(), Some(types::bool())),
            (
                &hex!("050707030b030b"),
                data::pair(vec![data::unit(), data::unit()]),
                Some(types::pair(vec![types::unit(), types::unit()])),
            ),
            (
                &hex!("050707030b0707030b030b"),
                data::pair(vec![data::unit(), data::unit(), data::unit()]),
                Some(types::pair(vec![
                    types::unit(),
                    types::unit(),
                    types::unit(),
                ])),
            ),
            (
                &hex!("050508030b"),
                data::right(data::unit()),
                Some(types::or(types::bool(), types::unit())),
            ),
            (
                &hex!("0505080707030b0707030b030b"),
                data::right(data::pair(vec![data::unit(), data::unit(), data::unit()])),
                Some(types::or(
                    types::bool(),
                    types::pair(vec![types::unit(), types::unit(), types::unit()]),
                )),
            ),
            (
                &hex!("050509030b"),
                data::some(data::unit()),
                Some(types::option(types::unit())),
            ),
            (
                &hex!("0505090707030b0707030b030b"),
                data::some(data::pair(vec![data::unit(), data::unit(), data::unit()])),
                Some(types::option(types::pair(vec![
                    types::unit(),
                    types::unit(),
                    types::unit(),
                ]))),
            ),
            (&hex!("05030a"), data::r#true(), Some(types::bool())),
            (&hex!("05030b"), data::unit(), Some(types::unit())),
        ]
    }

    fn sequence_values() -> Vec<(&'static [u8], Micheline, Option<Micheline>)> {
        vec![
            (
                &hex!("050200000000"),
                vec![].into(),
                Some(types::list(types::unit())),
            ),
            (
                &hex!("0502000000020000"),
                vec![data::int(0)].into(),
                Some(types::list(types::int())),
            ),
            (
                &hex!("050200000006030b030b030b"),
                vec![data::unit(), data::unit(), data::unit()].into(),
                Some(types::list(types::unit())),
            ),
            (
                &hex!("0502000000060704030b030b"),
                data::map(vec![data::elt(data::unit(), data::unit())]),
                Some(types::map(types::unit(), types::unit())),
            ),
            (
                &hex!("0502000000060704030b030b"),
                data::map(vec![data::elt(data::unit(), data::unit())]),
                Some(types::big_map(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000002030c"),
                vec![data::instructions::pack()].into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("0502000000070200000002030c"),
                vec![vec![data::instructions::pack()].into()].into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000009051f0200000002034f"),
                vec![data::instructions::dip(
                    vec![data::instructions::unit()].into(),
                    None,
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000010072c0200000002034f0200000002034f"),
                vec![data::instructions::r#if(
                    vec![data::instructions::unit()].into(),
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000010072d0200000002034f0200000002034f"),
                vec![data::instructions::r#if_cons(
                    vec![data::instructions::unit()].into(),
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000010072e0200000002034f0200000002034f"),
                vec![data::instructions::r#if_left(
                    vec![data::instructions::unit()].into(),
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000010072f0200000002034f0200000002034f"),
                vec![data::instructions::r#if_none(
                    vec![data::instructions::unit()].into(),
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("05020000001509310000000b036c036c0200000002034f00000000"),
                vec![data::instructions::lambda(
                    types::unit(),
                    types::unit(),
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("05020000000905340200000002034f"),
                vec![data::instructions::r#loop(
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("05020000000905380200000002034f"),
                vec![data::instructions::map(
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("05020000000905520200000002034f"),
                vec![data::instructions::iter(
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("05020000000905530200000002034f"),
                vec![data::instructions::loop_left(
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("05020000000f0743075e036c036c0200000002034f"),
                vec![data::instructions::push(
                    types::lambda(types::unit(), types::unit()),
                    data::sequence(vec![data::instructions::unit()]),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
            (
                &hex!("050200000015091d0000000b036c036c0200000002034f00000000"),
                vec![data::instructions::create_contract(
                    types::unit(),
                    types::unit(),
                    vec![data::instructions::unit()].into(),
                )]
                .into(),
                Some(types::lambda(types::unit(), types::unit())),
            ),
        ]
    }
}
