use crate::{
    internal::coder::{ConfigurableDecoder, ConfigurableEncoder, Decoder, Encoder},
    types::encoded::{ContractAddress, TraitMetaEncoded},
    Error,
};

use super::contract_hash_bytes_coder::{
    ContractHashBytesCoder, ContractHashBytesCoderConfiguration,
};

pub struct ContractAddressBytesCoder;

impl Encoder<ContractAddress, Vec<u8>, Error> for ContractAddressBytesCoder {
    fn encode(value: &ContractAddress) -> std::result::Result<Vec<u8>, Error> {
        let mut bytes = ContractHashBytesCoder::encode_with_configuration(
            &value.contract_hash().try_into()?,
            ContractHashBytesCoderConfiguration { with_suffix: true },
        )?;
        if let Some(entrypoint) = value.entrypoint() {
            bytes.extend_from_slice(entrypoint.as_bytes());
        }
        Ok(bytes)
    }
}

impl Decoder<ContractAddress, [u8], Error> for ContractAddressBytesCoder {
    fn decode(value: &[u8]) -> std::result::Result<ContractAddress, Error> {
        let meta = ContractAddress::meta_value();
        let (contract_hash_bytes, entrypoint_bytes) = value.split_at(meta.bytes_length + 1);
        let contract_hash = ContractHashBytesCoder::decode_with_configuration(
            contract_hash_bytes.to_vec().as_ref(),
            ContractHashBytesCoderConfiguration { with_suffix: true },
        )?;
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
