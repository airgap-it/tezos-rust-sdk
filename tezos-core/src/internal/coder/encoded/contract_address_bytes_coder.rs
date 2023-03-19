use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::ConsumableList,
    },
    types::encoded::{ContractAddress, ContractHash, Encoded, TraitMetaEncoded},
    Error, Result,
};
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContractAddressBytesCoder;

impl Encoder<ContractAddress, Vec<u8>, Error> for ContractAddressBytesCoder {
    fn encode(value: &ContractAddress) -> Result<Vec<u8>> {
        let contract_hash: ContractHash = value.contract_hash().try_into()?;
        let mut bytes = contract_hash.to_bytes()?;
        bytes.push(0);
        if let Some(entrypoint) = value.entrypoint() {
            bytes.extend_from_slice(entrypoint.as_bytes());
        }
        Ok(bytes)
    }
}

impl Decoder<ContractAddress, [u8], Error> for ContractAddressBytesCoder {
    fn decode(value: &[u8]) -> Result<ContractAddress> {
        let meta = ContractAddress::meta_value();
        let (contract_hash_bytes, entrypoint_bytes) = value.split_at(meta.bytes_length + 1);
        if !contract_hash_bytes.ends_with(&[0]) {
            return Err(Error::InvalidBytes);
        }
        let contract_hash_end_index = contract_hash_bytes.len() - 1;
        let contract_hash_bytes = &contract_hash_bytes[0..contract_hash_end_index];
        let contract_hash = ContractHash::from_bytes(contract_hash_bytes)?;
        let entrypoint = if entrypoint_bytes.is_empty() {
            None
        } else {
            Some(
                String::from_utf8(entrypoint_bytes.to_vec())
                    .map_err(|_error| Error::InvalidBytes)?,
            )
        };
        Ok(ContractAddress::from_components(
            &contract_hash,
            entrypoint.as_ref().map(|x| &**x),
        ))
    }
}

impl ConsumingDecoder<ContractAddress, u8, Error> for ContractAddressBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<ContractAddress> {
        let meta = ContractAddress::meta_value();
        let bytes = value.consume_until(meta.bytes_length + 1)?;
        if !bytes.ends_with(&[0]) {
            return Err(Error::InvalidBytes);
        }
        let contract_hash = ContractHash::from_bytes(&bytes[..meta.bytes_length])?;

        Ok(ContractAddress::from_components(&contract_hash, None))
    }
}
