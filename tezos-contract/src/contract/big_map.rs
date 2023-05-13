use tezos_core::{types::encoded::ScriptExprHash, Tezos};
use tezos_michelson::{
    micheline::Micheline,
    michelson::{
        types::{BigMap as BigMapType, Type},
        Michelson,
    },
    MichelinePacker,
};
use tezos_rpc::{
    client::TezosRpc,
    http::Http,
    models::{block::BlockId, contract::UnparsingMode},
};

use crate::{utils::AnyAnnotationValue, Result};

#[derive(Debug, Clone)]
pub struct BigMap {
    pub id: u32,
    pub name: Option<String>,
    pub key_type: Type,
    pub value_type: Type,
}

impl BigMap {
    pub(crate) fn new(big_map_type: BigMapType, id: u32) -> Self {
        let name: Option<String> = big_map_type
            .metadata()
            .any_annotation_value()
            .map(|name| name.into());
        BigMap {
            id,
            name,
            key_type: *big_map_type.key_type,
            value_type: *big_map_type.value_type,
        }
    }

    pub async fn get_value<HttpClient: Http>(
        &self,
        client: &TezosRpc<HttpClient>,
        key: Michelson,
        block_id: Option<&BlockId>,
    ) -> Result<Micheline> {
        let packed_key = key.pack(Some(&self.key_type))?;
        let hashed = Tezos::default().get_crypto().blake2b(&packed_key, 32)?;
        let script_expr: ScriptExprHash = (&hashed).try_into()?;
        let mut request = client
            .get_big_map_value(self.id, &script_expr)
            .unparsing_mode(UnparsingMode::Optimized_legacy);
        if let Some(block_id) = block_id {
            request = request.block_id(block_id);
        }
        let value = request.send().await?;
        let schema: Micheline = self.value_type.clone().into();

        Ok(MichelinePacker::post_unpack(value, &schema)?)
    }
}

#[derive(Debug, Clone)]
pub struct BigMapContainer {
    big_maps: Vec<BigMap>,
}

impl BigMapContainer {
    pub fn new(big_maps: Vec<BigMap>) -> Self {
        BigMapContainer { big_maps }
    }

    pub fn get_by_name(&self, name: &str) -> Option<&BigMap> {
        self.big_maps.iter().find(|big_map| {
            big_map
                .name
                .as_ref()
                .map_or(false, |big_map_name| big_map_name.eq(name))
        })
    }

    pub fn get_by_index(&self, index: usize) -> Option<&BigMap> {
        if index < self.big_maps.len() {
            return Some(&self.big_maps[index]);
        }

        None
    }
}
