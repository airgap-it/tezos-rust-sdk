use tezos_core::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::{ConsumableBytes, ConsumableList},
    },
    types::encoded::{BlockHash, Encoded},
};

use crate::{
    operations::{Operation, OperationContent, UnsignedOperation},
    Error, Result,
};

use super::operation_content_bytes_coder::OperationContentBytesCoder;

pub struct OperationBytesCoder;

impl<O: Operation> Encoder<O, Vec<u8>, Error> for OperationBytesCoder {
    fn encode(value: &O) -> std::result::Result<Vec<u8>, Error> {
        let branch_bytes = value.branch().to_bytes()?;
        let content_bytes = value.contents().into_iter().try_fold::<_, _, Result<_>>(
            Vec::<u8>::new(),
            |mut acc, value| {
                acc.append(&mut OperationContentBytesCoder::encode(value)?);
                Ok(acc)
            },
        )?;
        Ok([branch_bytes, content_bytes].concat())
    }
}

impl Decoder<UnsignedOperation, [u8], Error> for OperationBytesCoder {
    fn decode(value: &[u8]) -> Result<UnsignedOperation> {
        Self::decode_consuming(&mut ConsumableBytes::new(value))
    }
}

impl ConsumingDecoder<UnsignedOperation, u8, Error> for OperationBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<UnsignedOperation> {
        let branch = BlockHash::from_consumable_bytes(value)?;
        let mut contents = Vec::<OperationContent>::new();
        while !value.is_empty() {
            contents.push(OperationContentBytesCoder::decode_consuming(value)?);
        }
        Ok(UnsignedOperation::new(branch, contents))
    }
}
