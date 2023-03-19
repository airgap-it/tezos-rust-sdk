use crate::internal::{
    coder::{ConfigurableEncoder, ConsumingDecoder, Decoder, Encoder},
    consumable_list::ConsumableList,
};
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};
use alloc::vec::Vec;

pub struct EncodedBytesCoder;

impl EncodedBytesCoder {
    pub fn decode_with_meta<E: Encoded>(value: &[u8], meta: &MetaEncoded) -> Result<E> {
        if value.len() == meta.bytes_length {
            let bytes = [meta.bytes_prefix(), value].concat();
            return E::new(
                bs58::encode(bytes)
                    .with_check_version(meta.version())
                    .into_string(),
            );
        } else if value.starts_with(meta.versioned_bytes_prefix())
            && value.len() == (meta.bytes_length + meta.versioned_bytes_prefix().len())
        {
            return E::new(
                bs58::encode(&value[1..])
                    .with_check_version(meta.version())
                    .into_string(),
            );
        }

        return Err(Error::InvalidBytes);
    }

    pub fn decode_consuming_with_meta<E: Encoded, CL: ConsumableList<u8>>(
        value: &mut CL,
        meta: &MetaEncoded,
    ) -> Result<E> {
        let bytes = value.consume_until(meta.bytes_length)?;
        Self::decode_with_meta(&bytes, meta)
    }
}

impl<E: Encoded> ConfigurableEncoder<E, Vec<u8>, EncodedBytesCoderConfiguration, Error>
    for EncodedBytesCoder
{
    fn encode_with_configuration(
        value: &E,
        configuration: EncodedBytesCoderConfiguration,
    ) -> Result<Vec<u8>> {
        let bytes = bs58::decode(value.value())
            .with_check(Some(value.meta().version()))
            .into_vec()?;
        if bytes.len() <= value.meta().versioned_bytes_prefix().len()
            || !bytes.starts_with(value.meta().versioned_bytes_prefix())
        {
            return Err(Error::InvalidBytes);
        }
        let start_index = if configuration.keep_prefix {
            0
        } else {
            value.meta().versioned_bytes_prefix().len()
        };
        Ok(bytes[start_index..].to_vec())
    }
}

impl<E: Encoded> Encoder<E, Vec<u8>, Error> for EncodedBytesCoder {
    fn encode(value: &E) -> Result<Vec<u8>> {
        Self::encode_with_configuration(
            value,
            EncodedBytesCoderConfiguration { keep_prefix: false },
        )
    }
}

impl<E: Encoded> Decoder<E, [u8], Error> for EncodedBytesCoder {
    fn decode(value: &[u8]) -> Result<E> {
        let meta = MetaEncoded::recognize_bytes(value)?;
        Self::decode_with_meta(value, meta)
    }
}

impl<E: Encoded> ConsumingDecoder<E, u8, Error> for EncodedBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<E> {
        let meta = MetaEncoded::recognize_consumable_bytes(value.inner_value())?;
        Self::decode_consuming_with_meta(value, meta)
    }
}

pub struct EncodedBytesCoderConfiguration {
    pub keep_prefix: bool,
}
