use crate::internal::coder::{ConfigurableEncoder, ConsumingDecoder, Decoder, Encoder};
use crate::internal::consumable_list::ConsumableList;
use crate::types::encoded::{Encoded, MetaEncoded};
use crate::{Error, Result};

pub struct EncodedBytesCoder;

impl EncodedBytesCoder {
    pub fn new() -> Self {
        EncodedBytesCoder {}
    }
}

impl EncodedBytesCoder {
    pub fn decode_with_meta<E: Encoded>(&self, value: &[u8], meta: &MetaEncoded) -> Result<E> {
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

    pub fn decode_consuming_with_meta<E: Encoded>(
        &self,
        value: &mut Vec<u8>,
        meta: &MetaEncoded,
    ) -> Result<E> {
        let bytes = value.consume_until(meta.bytes_length);
        self.decode_with_meta(&bytes, meta)
    }
}

impl<E: Encoded> ConfigurableEncoder<&E, Vec<u8>, EncoderConfiguration> for EncodedBytesCoder {
    fn encode_with_configuration(
        &self,
        value: &E,
        configuration: EncoderConfiguration,
    ) -> Result<Vec<u8>> {
        let bytes = bs58::decode(value.base58())
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

impl<E: Encoded> Encoder<&E, Vec<u8>> for EncodedBytesCoder {
    fn encode(&self, value: &E) -> Result<Vec<u8>> {
        self.encode_with_configuration(value, EncoderConfiguration { keep_prefix: false })
    }
}

impl<E: Encoded> Decoder<E, &[u8]> for EncodedBytesCoder {
    fn decode(&self, value: &[u8]) -> Result<E> {
        let meta = MetaEncoded::recognize_bytes(value)?;
        self.decode_with_meta(value, meta)
    }
}

impl<E: Encoded> ConsumingDecoder<E, u8> for EncodedBytesCoder {
    fn decode_consuming(&self, value: &mut Vec<u8>) -> Result<E> {
        let meta = MetaEncoded::recognize_consumable_bytes(value)?;
        self.decode_consuming_with_meta(value, meta)
    }
}

pub struct EncoderConfiguration {
    pub keep_prefix: bool,
}
