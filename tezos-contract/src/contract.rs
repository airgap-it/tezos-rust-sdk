mod big_map;
mod entrypoints;
mod storage;

use async_trait::async_trait;

use tezos_core::types::{
    encoded::{Address, ContractHash, ImplicitAddress},
    mutez::Mutez,
    number::{Int, Nat},
};
use tezos_michelson::{
    micheline::{primitive_application::PrimitiveApplication, Micheline},
    michelson::{
        data::{
            Bytes, Data, Instruction, Left, Map, Pair, Right, Sequence as DataSequence,
            Some as DataSome, String as MichelsonString,
        },
        types::{ComparableType, Parameter, Type},
        DataPrimitive, Primitive,
    },
    MichelinePacker,
};
use tezos_operation::operations::{Entrypoint, Parameters, Transaction};
use tezos_rpc::{
    client::TezosRpc,
    http::Http,
    models::{
        block::BlockId,
        contract::{ContractScript, UnparsingMode},
    },
};

use crate::{utils::AnyAnnotationValue, Error, Result};

use self::entrypoints::MappedEntrypoints;
pub use self::{
    big_map::{BigMap, BigMapContainer},
    entrypoints::EntrypointPathComponent,
    storage::Storage,
};

/// The [Contract] structure represents a tezos contract.
///
/// It allows to retrieve values from its storage, retrieve big map values,
/// and easily create tezos operation representing contract calls.
///
/// # Example
///
/// ```rust
/// use tezos_contract::{Contract, ContractFetcher};
/// use tezos_rpc::client::TezosRpc;
/// use tezos_core::types::{encoded::ContractHash, number::Nat};
/// use tezos_michelson::michelson::data::{pair, try_string};
///
/// async fn get_big_map_value() {
///     let rpc_url = "https://testnet-tezos.giganode.io";
///     let rpc = TezosRpc::new(rpc_url.into());
///     let contract_address: ContractHash = "KT1J4CiyWPmtFPXAjpgBezM5hoVHXHNzWBHK".try_into().unwrap();
///     let contract = rpc.contract_at(contract_address, None).await.unwrap();
///     let ledger = contract.storage().big_maps().get_by_name("ledger").unwrap();
///     let balance: Nat = ledger.get_value(&rpc, pair(vec![try_string("tz1YY1LvD6TFH4z74pvxPQXBjAKHE5tB5Q8f").unwrap(), 0u8.into()]), None).await.unwrap().try_into().unwrap();
/// }
///
/// ```
#[derive(Debug, Clone)]
pub struct Contract {
    address: ContractHash,
    storage: Storage,
    entrypoints: MappedEntrypoints,
}

impl Contract {
    pub fn address(&self) -> &ContractHash {
        &self.address
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub(crate) fn new(
        address: ContractHash,
        script: ContractScript,
        entrypoints: MappedEntrypoints,
    ) -> Result<Contract> {
        Ok(Contract {
            address,
            storage: Storage::new(script)?,
            entrypoints,
        })
    }

    pub fn call(
        &self,
        entrypoint: Entrypoint,
        arguments: Vec<(&str, Data)>,
    ) -> Result<PartialTransaction> {
        if let Some(entrypoint_type) = self.entrypoints.get(&entrypoint) {
            let mut args = arguments;
            let parameters = Parameters::new(
                entrypoint,
                entrypoint_type
                    .construct_parameter_value(&mut args)?
                    .normalized(),
            );
            return Ok(PartialTransaction::new(
                0u8.into(),
                (&self.address).into(),
                Some(parameters),
            ));
        }

        Err(Error::EntrypointNotFound)
    }

    pub fn get_entrypoint_at_path(&self, path: &[EntrypointPathComponent]) -> Option<Entrypoint> {
        self.entrypoints.get_entrypoint_at_path(path)
    }
}

#[derive(Debug, Clone)]
pub struct PartialTransaction {
    pub amount: Mutez,
    pub destination: Address,
    pub parameters: Option<Parameters>,
}

impl PartialTransaction {
    pub fn new(amount: Mutez, destination: Address, parameters: Option<Parameters>) -> Self {
        Self {
            amount,
            destination,
            parameters,
        }
    }

    pub fn complete_with(
        self,
        source: ImplicitAddress,
        counter: Nat,
        fee: Option<Mutez>,
        amount: Option<Mutez>,
    ) -> Transaction {
        Transaction::new(
            source,
            fee.unwrap_or(0u8.into()),
            counter,
            0u8.into(),
            0u8.into(),
            amount.unwrap_or(0u8.into()),
            self.destination,
            self.parameters,
        )
    }
}

trait ParametersValueConstructor {
    fn construct_parameter_value(&self, arguments: &mut Vec<(&str, Data)>) -> Result<Micheline>;
}

impl ParametersValueConstructor for Type {
    fn construct_parameter_value(&self, arguments: &mut Vec<(&str, Data)>) -> Result<Micheline> {
        if let Some(key) = self.metadata().any_annotation_value() {
            if let Some(matching_arg_index) = arguments.iter().position(|arg| arg.0.eq(key)) {
                let value = arguments.remove(matching_arg_index).1.normalized();
                if value.is_compatible_with(self) {
                    let schema: Micheline = self.into();
                    return Ok(MichelinePacker::pre_pack(value.into(), &schema)?);
                } else {
                    return Err(Error::IncompatibleValue {
                        description: format!("{:?} is incompative for type: {:?}", value, self),
                    });
                }
            }
        }
        if let Some(first) = arguments.first().cloned() {
            let value = first.1.normalized();
            if first.0.is_empty() && value.is_compatible_with(self) {
                arguments.remove(0);
                let schema: Micheline = self.into();
                return Ok(MichelinePacker::pre_pack(value.into(), &schema)?);
            }
        }
        match self {
            Self::Pair(pair) => {
                let args = (&pair.types)
                    .iter()
                    .map(|pair_type| pair_type.construct_parameter_value(arguments))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(PrimitiveApplication::new(
                    Primitive::Data(DataPrimitive::Pair).into(),
                    Some(args),
                    None,
                )
                .into());
            }
            Self::List(list) => {
                let value = (&list.r#type).construct_parameter_value(arguments)?;
                Ok(Micheline::Sequence(vec![value].into()))
            }
            Self::Set(set) => {
                let value = (&set.r#type).construct_parameter_value(arguments)?;
                Ok(Micheline::Sequence(vec![value].into()))
            }
            Self::Map(map) => {
                let key = (&map.key_type).construct_parameter_value(arguments)?;
                let value = (&map.value_type).construct_parameter_value(arguments)?;
                Ok(Micheline::Sequence(
                    vec![PrimitiveApplication::new(
                        Primitive::Data(DataPrimitive::Elt).into(),
                        Some(vec![key, value]),
                        None,
                    )
                    .into()]
                    .into(),
                ))
            }
            Type::BigMap(big_map) => {
                let key = (&big_map.key_type).construct_parameter_value(arguments)?;
                let value = (&big_map.value_type).construct_parameter_value(arguments)?;
                Ok(Micheline::Sequence(
                    vec![PrimitiveApplication::new(
                        Primitive::Data(DataPrimitive::Elt).into(),
                        Some(vec![key, value]),
                        None,
                    )
                    .into()]
                    .into(),
                ))
            }
            _ => Err(Error::IncompatibleValue {
                description: format!("Could not construct value type: {:?}", self),
            }),
        }
    }
}

impl ParametersValueConstructor for ComparableType {
    fn construct_parameter_value(&self, arguments: &mut Vec<(&str, Data)>) -> Result<Micheline> {
        let r#type: Type = self.clone().into();
        r#type.construct_parameter_value(arguments)
    }
}

trait CompatibleWith<T> {
    fn is_compatible_with(&self, value: &T) -> bool;
}

trait StaticCompatibleWith<T> {
    fn is_compatible_with(value: &T) -> bool;
}

impl CompatibleWith<Type> for Data {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match self {
            Self::Int(_) => Int::is_compatible_with(value),
            Self::Nat(_) => Nat::is_compatible_with(value),
            Self::String(_) => MichelsonString::is_compatible_with(value),
            Self::Bytes(_) => Bytes::is_compatible_with(value),
            Self::Sequence(sequence) => sequence.is_compatible_with(value),
            Self::Map(map) => map.is_compatible_with(value),
            Self::Instruction(instruction) => instruction.is_compatible_with(value),
            Self::Unit(_) => match value {
                Type::Comparable(ComparableType::Unit(_)) => true,
                _ => false,
            },
            Self::True(_) | Self::False(_) => match value {
                Type::Comparable(ComparableType::Bool(_)) => true,
                _ => false,
            },
            Self::Pair(pair) => pair.is_compatible_with(value),
            Self::Left(left) => left.is_compatible_with(value),
            Self::Right(right) => right.is_compatible_with(value),
            Self::Some(some) => some.is_compatible_with(value),
            Self::None(_) => match value {
                Type::Option(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl StaticCompatibleWith<Type> for Int {
    fn is_compatible_with(value: &Type) -> bool {
        match value {
            Type::Comparable(ComparableType::Int(_))
            | Type::Comparable(ComparableType::Timestamp(_)) => true,
            _ => false,
        }
    }
}

impl StaticCompatibleWith<Type> for Nat {
    fn is_compatible_with(value: &Type) -> bool {
        match value {
            Type::Comparable(ComparableType::Nat(_))
            | Type::Comparable(ComparableType::Mutez(_)) => true,
            _ => false,
        }
    }
}

impl StaticCompatibleWith<Type> for MichelsonString {
    fn is_compatible_with(value: &Type) -> bool {
        match value {
            Type::Comparable(value) => match value {
                ComparableType::String(_)
                | ComparableType::ChainId(_)
                | ComparableType::KeyHash(_)
                | ComparableType::Key(_)
                | ComparableType::Signature(_)
                | ComparableType::Timestamp(_)
                | ComparableType::Address(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl StaticCompatibleWith<Type> for Bytes {
    fn is_compatible_with(value: &Type) -> bool {
        match value {
            Type::Comparable(ComparableType::Bytes(_)) => true,
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for DataSequence {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match value {
            Type::List(value) => {
                let element_type = &*value.r#type;
                self.values()
                    .iter()
                    .all(|item| item.is_compatible_with(element_type))
            }
            Type::Set(value) => {
                let element_type = &value.r#type;
                self.values()
                    .iter()
                    .all(|item| item.is_compatible_with(&element_type))
            }
            Type::Pair(_) => {
                let pair: Data = Pair::new(self.clone().into_values()).into();
                pair.is_compatible_with(value)
            }
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for Map {
    fn is_compatible_with(&self, value: &Type) -> bool {
        fn check(map: &Map, key_type: &Type, value_type: &Type) -> bool {
            map.values().iter().all(|item| {
                let key = &*item.key;
                let value = &*item.value;

                return key.is_compatible_with(key_type) && value.is_compatible_with(value_type);
            })
        }

        match value {
            Type::Map(map) => check(self, &*map.key_type, &*map.value_type),
            Type::BigMap(map) => check(self, &*map.key_type, &*map.value_type),
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for Instruction {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match self {
            Self::Sequence(_) => match value {
                Type::Lambda(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for Pair {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match value {
            Type::Pair(pair) => {
                if pair.types.len() != self.values.len() {
                    return false;
                }

                pair.types
                    .iter()
                    .zip(self.values.iter())
                    .all(|(r#type, value)| value.is_compatible_with(r#type))
            }
            Type::Comparable(ComparableType::Pair(pair)) => {
                if pair.types.len() != self.values.len() {
                    return false;
                }

                pair.types
                    .iter()
                    .zip(self.values.iter())
                    .all(|(r#type, value)| value.is_compatible_with(&r#type.clone().into()))
            }
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for Left {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match value {
            Type::Or(or) => self.value.is_compatible_with(&or.lhs),
            Type::Comparable(ComparableType::Or(or)) => {
                self.value.is_compatible_with(&(*or.lhs).clone().into())
            }
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for Right {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match value {
            Type::Or(or) => self.value.is_compatible_with(&or.rhs),
            Type::Comparable(ComparableType::Or(or)) => {
                self.value.is_compatible_with(&(*or.rhs).clone().into())
            }
            _ => false,
        }
    }
}

impl CompatibleWith<Type> for DataSome {
    fn is_compatible_with(&self, value: &Type) -> bool {
        match value {
            Type::Option(option) => self.value.is_compatible_with(&option.r#type),
            Type::Comparable(ComparableType::Option(option)) => self
                .value
                .is_compatible_with(&(*option.r#type).clone().into()),
            _ => false,
        }
    }
}

#[async_trait]
pub trait ContractFetcher {
    async fn contract_at(
        &self,
        address: ContractHash,
        block_id: Option<&BlockId>,
    ) -> Result<Contract>;
}

#[async_trait]
impl<HttpClient: Http + Sync> ContractFetcher for TezosRpc<HttpClient> {
    async fn contract_at(
        &self,
        address: ContractHash,
        block_id: Option<&BlockId>,
    ) -> Result<Contract> {
        let generic_address: Address = (&address).into();
        let mut request = self
            .get_contract_script(&generic_address)
            .unparsing_mode(UnparsingMode::Optimized_legacy);
        if let Some(block_id) = block_id {
            request = request.block_id(block_id);
        }
        let script = request.send().await?;
        let parameter: Parameter = script
            .code
            .values()
            .iter()
            .nth(0)
            .ok_or(Error::InvalidContractScript)?
            .clone()
            .try_into()?;
        let entrypoints = MappedEntrypoints::new(parameter)?;
        Contract::new(address, script, entrypoints)
    }
}
