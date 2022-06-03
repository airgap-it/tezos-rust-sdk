use crate::{
    types::{mutez::Mutez, number::natural::Natural},
    Result,
};

use super::{number::natural::NaturalBytesCoder, ConsumingDecoder, Decoder, Encoder};

pub struct MutezBytesCoder {
    coder: NaturalBytesCoder,
}

impl MutezBytesCoder {
    pub fn new() -> Self {
        MutezBytesCoder {
            coder: NaturalBytesCoder::new(),
        }
    }
}

impl Encoder<&Mutez, Vec<u8>> for MutezBytesCoder {
    fn encode(&self, value: &Mutez) -> Result<Vec<u8>> {
        let value: Natural = value.into();
        self.coder.encode(&value)
    }
}

impl Decoder<Mutez, &Vec<u8>> for MutezBytesCoder {
    fn decode(&self, value: &Vec<u8>) -> Result<Mutez> {
        let nat = self.coder.decode(value)?;
        (&nat).try_into()
    }
}

impl ConsumingDecoder<Mutez, u8> for MutezBytesCoder {
    fn decode_consuming(&self, value: &mut Vec<u8>) -> Result<Mutez> {
        let nat = self.coder.decode_consuming(value)?;
        (&nat).try_into()
    }
}
