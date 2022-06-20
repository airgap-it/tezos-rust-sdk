use crate::{
    types::{mutez::Mutez, number::natural::Natural},
    Error, Result,
};

use super::{number::natural::NaturalBytesCoder, ConsumingDecoder, Decoder, Encoder};

pub struct MutezBytesCoder;

impl Encoder<Mutez, Vec<u8>, Error> for MutezBytesCoder {
    fn encode(value: &Mutez) -> Result<Vec<u8>> {
        let value: Natural = value.into();
        NaturalBytesCoder::encode(&value)
    }
}

impl Decoder<Mutez, Vec<u8>, Error> for MutezBytesCoder {
    fn decode(value: &Vec<u8>) -> Result<Mutez> {
        let nat = NaturalBytesCoder::decode(value)?;
        (&nat).try_into()
    }
}

impl ConsumingDecoder<Mutez, u8, Error> for MutezBytesCoder {
    fn decode_consuming(value: &mut Vec<u8>) -> Result<Mutez> {
        let nat = NaturalBytesCoder::decode_consuming(value)?;
        (&nat).try_into()
    }
}
