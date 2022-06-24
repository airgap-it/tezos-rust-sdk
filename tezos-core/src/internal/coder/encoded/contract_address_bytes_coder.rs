use crate::{
    internal::coder::{ConsumingDecoder, Decoder, Encoder},
    types::encoded::ContractAddress,
    Error,
};

pub struct ContractAddressBytesCoder;

impl Encoder<ContractAddress, Vec<u8>, Error> for ContractAddressBytesCoder {
    fn encode(value: &ContractAddress) -> std::result::Result<Vec<u8>, Error> {
        todo!()
    }
}

impl Decoder<ContractAddress, Vec<u8>, Error> for ContractAddressBytesCoder {
    fn decode(value: &Vec<u8>) -> Result<ContractAddress, Error> {
        todo!()
    }
}

impl ConsumingDecoder<ContractAddress, u8, Error> for ContractAddressBytesCoder {
    fn decode_consuming(value: &mut Vec<u8>) -> Result<ContractAddress, Error> {
        todo!()
    }
}
