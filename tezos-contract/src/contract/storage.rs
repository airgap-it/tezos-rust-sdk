use std::collections::HashMap;

use tezos_core::{internal::traits::InnerValueRef, types::number::Int};
use tezos_michelson::{
    micheline::Micheline,
    michelson::{
        data::{Data, Pair as DataPair},
        types::{BigMap as TypeBigMap, Pair as TypePair, Storage as TypeStorage, Type},
    },
    MichelinePacker,
};
use tezos_rpc::models::contract::ContractScript;

use crate::{utils::AnyAnnotationValue, Error, Result};

use super::big_map::{BigMap, BigMapContainer};

#[derive(Debug, Clone)]
pub struct Storage {
    big_maps: BigMapContainer,
    mapped: MappedStorage,
}

impl Storage {
    pub fn big_maps(&self) -> &BigMapContainer {
        &self.big_maps
    }

    pub fn new(script: ContractScript) -> Result<Self> {
        let storage_type: TypeStorage = script
            .code
            .normalized()
            .into_values()
            .into_iter()
            .nth(1)
            .ok_or(Error::InvalidContractScript)?
            .try_into()?;
        let storage_type = *storage_type.r#type;
        let storage_value: Data = script.storage.normalized().try_into()?;
        let mut big_maps = Vec::<BigMap>::new();
        let mapped = MappedStorage::new(storage_type, storage_value, |big_map_type, id| {
            big_maps.push(BigMap::new(big_map_type, id));
        })?;
        Ok(Self {
            big_maps: BigMapContainer::new(big_maps),
            mapped,
        })
    }

    pub fn get_at_index(&self, index: usize) -> Option<Data> {
        self.mapped.get_at_index(index)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Data> {
        self.mapped.get_by_name(name)
    }
}

#[derive(Debug, Clone)]
struct MappedStorage {
    r#type: Type,
    value: Data,
    mapped_values: HashMap<String, Data>,
}

impl MappedStorage {
    fn new<'a, F>(storage_type: Type, storage_value: Data, mut big_map_handler: F) -> Result<Self>
    where
        F: FnMut(TypeBigMap, u32),
    {
        let mut mapped_values = HashMap::<String, Data>::new();

        let mut to_visit_types = vec![&storage_type];
        let mut to_visit_values = vec![&storage_value];

        while let (Some(current_type), Some(current_value)) =
            (to_visit_types.pop(), to_visit_values.pop())
        {
            let big_map_type: Option<&TypeBigMap> = current_type.inner_value_ref();
            if let Some(big_map_type) = big_map_type {
                let int: Option<&Int> = current_value.inner_value_ref();
                if let Some(id) = int {
                    big_map_handler(big_map_type.clone(), id.to_integer()?);
                }
            }

            if let Some(name) = current_type.metadata().any_annotation_value() {
                let schema: Micheline = current_type.into();
                if let Ok(unpacked_value) =
                    MichelinePacker::post_unpack(current_value.into(), &schema)
                {
                    let data: std::result::Result<Data, _> = unpacked_value.try_into();
                    if let Ok(data) = data {
                        mapped_values.insert(name.into(), data);
                    }
                }
            }

            let value_pair: Option<&DataPair> = current_value.inner_value_ref();
            let type_pair: Option<&TypePair> = current_type.inner_value_ref();
            match (value_pair, type_pair) {
                (Some(value_pair), Some(type_pair)) => {
                    to_visit_types.extend(type_pair.types.iter().rev());
                    to_visit_values.extend(value_pair.values.iter().rev());
                }
                _ => {}
            }
        }

        Ok(MappedStorage {
            r#type: storage_type,
            value: storage_value,
            mapped_values,
        })
    }

    fn get_at_index(&self, index: usize) -> Option<Data> {
        if let (Some(r#type), Some(value)) = (
            self.get_type_at_index(index),
            self.get_value_at_index(index),
        ) {
            let data: std::result::Result<Option<Data>, _> =
                MichelinePacker::post_unpack(value.into(), &r#type.into())
                    .ok()
                    .map(|unpacked| unpacked.try_into())
                    .map_or(Ok(None), |r| r.map(Some));
            return data.ok().flatten();
        }

        None
    }

    fn get_by_name(&self, name: &str) -> Option<&Data> {
        self.mapped_values.get(name)
    }

    fn get_type_at_index(&self, index: usize) -> Option<&Type> {
        self.r#type.get_by_flattened_index(index)
    }

    fn get_value_at_index(&self, index: usize) -> Option<&Data> {
        self.value.get_by_flattened_index(index)
    }
}

trait GetByFalltenedIndex {
    fn get_by_flattened_index(&self, index: usize) -> Option<&Self>;
    fn flattened_count(&self) -> usize;
}

impl GetByFalltenedIndex for Type {
    fn get_by_flattened_index(&self, index: usize) -> Option<&Self> {
        match self {
            Self::Pair(pair) => {
                let mut current_index = index;
                for arg in &pair.types {
                    if let Some(result) = arg.get_by_flattened_index(current_index) {
                        return Some(result);
                    }
                    if current_index == 0 {
                        break;
                    }
                    current_index -= arg.flattened_count();
                }
                None
            }
            _ => {
                if index == 0 {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    fn flattened_count(&self) -> usize {
        match self {
            Self::Pair(pair) => pair
                .types
                .iter()
                .fold(0, |acc, arg| acc + arg.flattened_count()),
            _ => 1,
        }
    }
}

impl GetByFalltenedIndex for Data {
    fn get_by_flattened_index(&self, index: usize) -> Option<&Self> {
        match self {
            Data::Pair(pair) => {
                let mut current_index = index;
                for arg in &pair.values {
                    if let Some(result) = arg.get_by_flattened_index(current_index) {
                        return Some(result);
                    }
                    if current_index == 0 {
                        break;
                    }
                    current_index -= arg.flattened_count();
                }
                None
            }
            Data::Int(_) | Data::Nat(_) | Data::String(_) | Data::Bytes(_) | Data::Sequence(_) => {
                if index == 0 {
                    Some(self)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn flattened_count(&self) -> usize {
        match self {
            Self::Pair(pair) => pair
                .values
                .iter()
                .fold(0, |acc, arg| acc + arg.flattened_count()),
            _ => 1,
        }
    }
}
