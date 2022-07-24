use async_trait::async_trait;

use tezos_core::types::{
    encoded::{Address, ContractHash, ImplicitAddress},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::{
    micheline::Micheline,
    michelson::{
        data::Data,
        types::{Parameter, Type},
    },
};
use tezos_operation::operations::{Entrypoint, Parameters, Transaction};
use tezos_rpc::{
    client::TezosRpc,
    http::Http,
    models::{block::BlockId, contract::UnparsingMode},
};

use crate::{
    entrypoints::{EntrypointPath, MappedEntrypoints},
    storage::Storage,
    Error, Result,
};

#[derive(Debug, Clone)]
pub struct Contract<'a, HttpClient: Http> {
    pub address: ContractHash,
    pub storage: Storage<'a, HttpClient>,
    pub client: &'a TezosRpc<HttpClient>,

    entrypoints: MappedEntrypoints,
}

impl<'a, HttpClient: Http> Contract<'a, HttpClient> {
    pub(crate) async fn new(
        address: ContractHash,
        client: &'a TezosRpc<HttpClient>,
        block_id: Option<&BlockId>,
    ) -> Result<Contract<'a, HttpClient>> {
        let generic_address: Address = (&address).into();
        let mut request = client
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
        return Ok(Contract {
            address,
            storage: Storage::new(script, client)?,
            client,
            entrypoints,
        });
    }

    pub fn call(
        &self,
        entrypoint: Entrypoint,
        arguments: &[(&str, Data)],
    ) -> Result<PartialTransaction> {
        if let Some(entrypoint_type) = self.entrypoints.get(&entrypoint) {
            let mut args = arguments.to_vec();
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

    pub fn get_entrypoint_at_path(&self, path: &[EntrypointPath]) -> Option<Entrypoint> {
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
        todo!()
    }
}

#[async_trait]
pub trait ContractFetcher<'a, HttpClient: Http + Sync> {
    async fn contract_at(
        &'a self,
        address: ContractHash,
        block_id: Option<&BlockId>,
    ) -> Result<Contract<'a, HttpClient>>;
}

#[async_trait]
impl<'a, HttpClient: Http + Sync> ContractFetcher<'a, HttpClient> for TezosRpc<HttpClient> {
    async fn contract_at(
        &'a self,
        address: ContractHash,
        block_id: Option<&BlockId>,
    ) -> Result<Contract<'a, HttpClient>> {
        Contract::<'a, HttpClient>::new(address, self, block_id).await
    }
}

// pub fn construct_value(
//     &self,
//     arguments: &mut Vec<(&str, MichelsonV1Expression)>,
// ) -> Result<MichelsonV1Expression> {
//     let prim = self.prim().ok_or(Error::InvalidType)?;
//     let type_ = prim.get_type().ok_or(Error::InvalidType)?;
//     if let Some(key) = prim.first_annotation(&[AnnotationType::Field, AnnotationType::Type]) {
//         if let Some(matching_arg_index) = arguments.iter().position(|arg| arg.0.eq(&key)) {
//             let value = arguments.remove(matching_arg_index).1;
//             if value.is_compatible_with(self) {
//                 return Ok(value.pre_pack(self)?);
//             } else {
//                 return Err(Error::InvalidType);
//             }
//         }
//     }
//     if let Some(first) = arguments.first().cloned() {
//         if first.0.is_empty() && first.1.is_compatible_with(self) {
//             arguments.remove(0);
//             return Ok(first.1.pre_pack(self)?);
//         }
//     }
//     match type_ {
//         Type::Pair => {
//             if let Some(pair_types) = &prim.args {
//                 let args = pair_types
//                     .iter()
//                     .map(|pair_type| pair_type.construct_value(arguments))
//                     .collect::<Result<Vec<_>>>()?;
//                 return Ok(data::prim(Data::Pair, Some(args)));
//             }
//             return Err(Error::InvalidType);
//         }
//         Type::List | Type::Set => {
//             if let Some(element_type) = prim.args.as_ref().and_then(|args| args.first()) {
//                 let value = element_type.construct_value(arguments)?;
//                 return Ok(seq(vec![value]));
//             }
//             return Err(Error::InvalidType);
//         }
//         Type::Map | Type::BigMap => {
//             if let Some(map_types) = &prim.args {
//                 if let Some(key_type) = map_types.first() {
//                     if let Some(value_type) = map_types.last() {
//                         let key = key_type.construct_value(arguments)?;
//                         let value = value_type.construct_value(arguments)?;
//                         return Ok(seq(vec![data::prim(Data::Elt, Some(vec![key, value]))]));
//                     }
//                 }
//             }
//             return Err(Error::InvalidType);
//         }
//         _ => Err(Error::InvalidType),
//     }
// }
