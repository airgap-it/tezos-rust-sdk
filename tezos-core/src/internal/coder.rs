pub mod encoded;
pub mod mutez;
pub mod number;

use crate::Result;

pub trait ConfigurableEncoder<T, S, C> {
    fn encode_with_configuration(&self, value: T, configuration: C) -> Result<S>;
}

pub trait Encoder<T, S>: ConfigurableEncoder<T, S, ()> {
    fn encode(&self, value: T) -> Result<S>;
}

impl<E, T, S> ConfigurableEncoder<T, S, ()> for E
where
    E: Encoder<T, S>,
{
    fn encode_with_configuration(&self, value: T, _configuration: ()) -> Result<S> {
        self.encode(value)
    }
}

pub trait ConfigurableDecoder<T, S, C> {
    fn decode_with_configuration(&self, value: S, configuration: C) -> Result<T>;
}

pub trait Decoder<T, S>: ConfigurableDecoder<T, S, ()> {
    fn decode(&self, value: S) -> Result<T>;
}

impl<D, T, S> ConfigurableDecoder<T, S, ()> for D
where
    D: Decoder<T, S>,
{
    fn decode_with_configuration(&self, value: S, _configuration: ()) -> Result<T> {
        self.decode(value)
    }
}

pub trait ConfigurableConsumingDecoder<T, S, C> {
    fn decode_consuming_with_configuration(
        &self,
        value: &mut Vec<S>,
        configuration: C,
    ) -> Result<T>;
}

pub trait ConsumingDecoder<T, S>: ConfigurableConsumingDecoder<T, S, ()> {
    fn decode_consuming(&self, value: &mut Vec<S>) -> Result<T>;
}

impl<D, T, S> ConfigurableConsumingDecoder<T, S, ()> for D
where
    D: ConsumingDecoder<T, S>,
{
    fn decode_consuming_with_configuration(
        &self,
        value: &mut Vec<S>,
        _configuration: (),
    ) -> Result<T> {
        self.decode_consuming(value)
    }
}
