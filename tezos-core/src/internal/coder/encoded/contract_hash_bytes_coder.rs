use crate::{
    internal::{
        coder::{
            ConfigurableConsumingDecoder, ConfigurableDecoder, ConfigurableEncoder,
            ConsumingDecoder, Decoder, EncodedBytesCoder, Encoder,
        },
        consumable_list::ConsumableList,
    },
    types::encoded::{ContractHash, MetaEncoded, TraitMetaEncoded},
    Error, Result,
};

pub struct ContractHashBytesCoder;

impl ContractHashBytesCoder {
    pub fn decode_with_meta(value: &[u8], _meta: &MetaEncoded) -> Result<ContractHash> {
        Self::decode_with_configuration(
            &value.to_vec(),
            ContractHashBytesCoderConfiguration { with_suffix: false },
        )
    }

    pub fn decode_consuming_with_meta(
        value: &mut Vec<u8>,
        _meta: &MetaEncoded,
    ) -> Result<ContractHash> {
        Self::decode_consuming_with_configuration(
            value,
            ContractHashBytesCoderConfiguration { with_suffix: false },
        )
    }
}

impl ConfigurableEncoder<ContractHash, Vec<u8>, ContractHashBytesCoderConfiguration, Error>
    for ContractHashBytesCoder
{
    fn encode_with_configuration(
        value: &ContractHash,
        configuration: ContractHashBytesCoderConfiguration,
    ) -> std::result::Result<Vec<u8>, Error> {
        let mut bytes = EncodedBytesCoder::encode(value)?;
        if configuration.with_suffix {
            bytes.push(0);
        }
        Ok(bytes)
    }
}

impl Encoder<ContractHash, Vec<u8>, Error> for ContractHashBytesCoder {
    fn encode(value: &ContractHash) -> std::result::Result<Vec<u8>, Error> {
        Self::encode_with_configuration(
            value,
            ContractHashBytesCoderConfiguration { with_suffix: false },
        )
    }
}

impl ConfigurableDecoder<ContractHash, Vec<u8>, ContractHashBytesCoderConfiguration, Error>
    for ContractHashBytesCoder
{
    fn decode_with_configuration(
        value: &Vec<u8>,
        configuration: ContractHashBytesCoderConfiguration,
    ) -> Result<ContractHash> {
        if configuration.with_suffix && !value.ends_with(&[0]) {
            return Err(Error::InvalidBytes);
        }
        let end_index = if configuration.with_suffix {
            value.len() - 1
        } else {
            value.len()
        };
        EncodedBytesCoder::decode_with_meta(&value[..end_index], ContractHash::meta_value())
    }
}

impl ConfigurableConsumingDecoder<ContractHash, u8, ContractHashBytesCoderConfiguration, Error>
    for ContractHashBytesCoder
{
    fn decode_consuming_with_configuration(
        value: &mut Vec<u8>,
        configuration: ContractHashBytesCoderConfiguration,
    ) -> Result<ContractHash> {
        let meta = ContractHash::meta_value();
        let end_index = if configuration.with_suffix {
            meta.bytes_length + 1
        } else {
            meta.bytes_length
        };
        let bytes = value.consume_until(end_index)?;
        Self::decode_with_configuration(&bytes, configuration)
    }
}

impl Decoder<ContractHash, Vec<u8>, Error> for ContractHashBytesCoder {
    fn decode(value: &Vec<u8>) -> std::result::Result<ContractHash, Error> {
        Self::decode_with_configuration(
            value,
            ContractHashBytesCoderConfiguration { with_suffix: false },
        )
    }
}

impl ConsumingDecoder<ContractHash, u8, Error> for ContractHashBytesCoder {
    fn decode_consuming(value: &mut Vec<u8>) -> std::result::Result<ContractHash, Error> {
        Self::decode_consuming_with_configuration(
            value,
            ContractHashBytesCoderConfiguration { with_suffix: false },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ContractHashBytesCoderConfiguration {
    pub with_suffix: bool,
}
