use super::{number::natural::NaturalBytesCoder, ConsumingDecoder, Decoder, Encoder};
use crate::{
    internal::consumable_list::ConsumableList,
    types::{mutez::Mutez, number::Nat},
    Error, Result,
};
use alloc::vec::Vec;

pub struct MutezBytesCoder;

impl Encoder<Mutez, Vec<u8>, Error> for MutezBytesCoder {
    fn encode(value: &Mutez) -> Result<Vec<u8>> {
        let value: Nat = value.into();
        NaturalBytesCoder::encode(&value)
    }
}

impl Decoder<Mutez, [u8], Error> for MutezBytesCoder {
    fn decode(value: &[u8]) -> Result<Mutez> {
        let nat = NaturalBytesCoder::decode(value)?;
        (&nat).try_into()
    }
}

impl ConsumingDecoder<Mutez, u8, Error> for MutezBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Mutez> {
        let nat = NaturalBytesCoder::decode_consuming(value)?;
        (&nat).try_into()
    }
}
