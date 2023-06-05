mod encoded;
mod mutez;
mod number;

pub use self::{
    encoded::{
        address_bytes_coder::AddressBytesCoder,
        contract_address_bytes_coder::ContractAddressBytesCoder,
        encoded_bytes_coder::{EncodedBytesCoder, EncodedBytesCoderConfiguration},
        encoded_group_bytes_coder::{EncodedGroupBytesCoder, TagProvider},
        implicit_address_bytes_coder::ImplicitAddressBytesCoder,
        public_key_bytes_coder::PublicKeyBytesCoder,
    },
    mutez::MutezBytesCoder,
    number::{integer::IntegerBytesCoder, natural::NaturalBytesCoder},
};

use super::consumable_list::ConsumableList;

pub trait ConfigurableEncoder<T, S, C, Error> {
    fn encode_with_configuration(value: &T, configuration: C) -> core::result::Result<S, Error>;
}

pub trait Encoder<T, S, Error>: ConfigurableEncoder<T, S, (), Error> {
    fn encode(value: &T) -> core::result::Result<S, Error>;
}

impl<E, T, S, Error> ConfigurableEncoder<T, S, (), Error> for E
where
    E: Encoder<T, S, Error>,
{
    fn encode_with_configuration(value: &T, _configuration: ()) -> Result<S, Error> {
        Self::encode(value)
    }
}

pub trait ConfigurableDecoder<T, S, C, Error>
where
    S: ?Sized,
{
    fn decode_with_configuration(value: &S, configuration: C) -> Result<T, Error>;
}

pub trait Decoder<T, S, Error>: ConfigurableDecoder<T, S, (), Error>
where
    S: ?Sized,
{
    fn decode(value: &S) -> Result<T, Error>;
}

impl<D, T, S, Error> ConfigurableDecoder<T, S, (), Error> for D
where
    D: Decoder<T, S, Error>,
    S: ?Sized,
{
    fn decode_with_configuration(value: &S, _configuration: ()) -> Result<T, Error> {
        Self::decode(value)
    }
}

pub trait ConfigurableConsumingDecoder<T, S, C, Error> {
    fn decode_consuming_with_configuration<CL: ConsumableList<S>>(
        value: &mut CL,
        configuration: C,
    ) -> Result<T, Error>;
}

pub trait ConsumingDecoder<T, S, Error>: ConfigurableConsumingDecoder<T, S, (), Error> {
    fn decode_consuming<CL: ConsumableList<S>>(value: &mut CL) -> Result<T, Error>;
}

impl<D, T, S, Error> ConfigurableConsumingDecoder<T, S, (), Error> for D
where
    D: ConsumingDecoder<T, S, Error>,
{
    fn decode_consuming_with_configuration<CL: ConsumableList<S>>(
        value: &mut CL,
        _configuration: (),
    ) -> Result<T, Error> {
        Self::decode_consuming(value)
    }
}
