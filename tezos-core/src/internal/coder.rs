mod encoded;
mod mutez;
mod number;

pub use self::{
    encoded::{
        address_bytes_coder::AddressBytesCoder,
        contract_address_bytes_coder::ContractAddressBytesCoder,
        contract_hash_bytes_coder::ContractHashBytesCoder,
        encoded_bytes_coder::{EncodedBytesCoder, EncodedBytesCoderConfiguration},
        encoded_group_bytes_coder::{EncodedGroupBytesCoder, TagProvider},
        implicit_address_bytes_coder::ImplicitAddressBytesCoder,
        public_key_bytes_coder::PublicKeyBytesCoder,
    },
    mutez::MutezBytesCoder,
    number::{integer::IntegerBytesCoder, natural::NaturalBytesCoder},
};

pub trait ConfigurableEncoder<T, S, C, Error> {
    fn encode_with_configuration(value: &T, configuration: C) -> std::result::Result<S, Error>;
}

pub trait Encoder<T, S, Error>: ConfigurableEncoder<T, S, (), Error> {
    fn encode(value: &T) -> std::result::Result<S, Error>;
}

impl<E, T, S, Error> ConfigurableEncoder<T, S, (), Error> for E
where
    E: Encoder<T, S, Error>,
{
    fn encode_with_configuration(value: &T, _configuration: ()) -> Result<S, Error> {
        Self::encode(value)
    }
}

pub trait ConfigurableDecoder<T, S, C, Error> {
    fn decode_with_configuration(value: &S, configuration: C) -> Result<T, Error>;
}

pub trait Decoder<T, S, Error>: ConfigurableDecoder<T, S, (), Error> {
    fn decode(value: &S) -> Result<T, Error>;
}

impl<D, T, S, Error> ConfigurableDecoder<T, S, (), Error> for D
where
    D: Decoder<T, S, Error>,
{
    fn decode_with_configuration(value: &S, _configuration: ()) -> Result<T, Error> {
        Self::decode(value)
    }
}

pub trait ConfigurableConsumingDecoder<T, S, C, Error> {
    fn decode_consuming_with_configuration(
        value: &mut Vec<S>,
        configuration: C,
    ) -> Result<T, Error>;
}

pub trait ConsumingDecoder<T, S, Error>: ConfigurableConsumingDecoder<T, S, (), Error> {
    fn decode_consuming(value: &mut Vec<S>) -> Result<T, Error>;
}

impl<D, T, S, Error> ConfigurableConsumingDecoder<T, S, (), Error> for D
where
    D: ConsumingDecoder<T, S, Error>,
{
    fn decode_consuming_with_configuration(
        value: &mut Vec<S>,
        _configuration: (),
    ) -> Result<T, Error> {
        Self::decode_consuming(value)
    }
}
