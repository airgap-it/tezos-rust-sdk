use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::{ConsumableBytes, ConsumableList},
    },
    types::number::Nat,
    Error, Result,
};
use alloc::{vec, vec::Vec};

#[derive(Debug)]
pub struct NaturalBytesCoder;

impl NaturalBytesCoder {
    pub fn encode_unsigned(value: BigUint) -> Vec<u8> {
        if value == BigUint::zero() {
            return vec![0];
        }
        Self::encode_with(value, vec![])
    }

    fn encode_with(value: BigUint, encoded: Vec<u8>) -> Vec<u8> {
        if value == BigUint::zero() {
            return encoded;
        }

        let byte = &value & BigUint::from(0b0111_1111u8);
        let next_value = &value >> 7u8;
        let sequence_mask = if next_value == BigUint::zero() {
            BigUint::from(0b0000_0000u8)
        } else {
            BigUint::from(0b1000_0000u8)
        };

        let encoded_byte = (byte | sequence_mask).to_u8().unwrap();

        Self::encode_with(next_value, [encoded, vec![encoded_byte]].concat())
    }

    fn decode_with<CL: ConsumableList<u8>>(
        value: &mut CL,
        decoded: BigUint,
        shift: u8,
    ) -> Result<BigUint> {
        let byte = value.consume_first()?;
        let part = BigUint::from(byte & 0b0111_1111u8);
        let has_next = (byte & 0b1000_0000) == 0b1000_0000;
        let decoded = decoded + (part << shift);
        if has_next {
            return Self::decode_with(value, decoded, shift + 7);
        }
        return Ok(decoded);
    }
}

impl Encoder<Nat, Vec<u8>, Error> for NaturalBytesCoder {
    fn encode(value: &Nat) -> Result<Vec<u8>> {
        let value: BigUint = value.to_integer().unwrap();
        if value == BigUint::zero() {
            return Ok(vec![0]);
        }
        Ok(Self::encode_with(value, vec![]))
    }
}

impl Decoder<Nat, [u8], Error> for NaturalBytesCoder {
    fn decode(value: &[u8]) -> Result<Nat> {
        let value = &mut ConsumableBytes::new(value);

        Self::decode_consuming(value)
    }
}

impl ConsumingDecoder<Nat, u8, Error> for NaturalBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Nat> {
        if value.is_empty() {
            return Err(Error::InvalidNaturalBytes);
        }
        let result = Self::decode_with(value, BigUint::zero(), 0)?;

        Ok(result.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_values() -> Result<Vec<(Nat, Vec<u8>)>> {
        Ok(vec![
            ((0u8).into(), vec![0]),
            ((1u8).into(), vec![1]),
            ((10u8).into(), vec![10]),
            ((42u8).into(), vec![42]),
            ((64u8).into(), vec![64]),
            ((127u8).into(), vec![127]),
            ((128u32).into(), vec![128, 1]),
            (
                (18756523543673u64).into(),
                vec![249, 152, 177, 191, 241, 161, 4],
            ),
            (
                (6852352674543413768u64).into(),
                vec![136, 212, 238, 142, 188, 206, 156, 140, 95],
            ),
            (
                "54576326575686358562454576456764".try_into()?,
                vec![
                    188, 200, 169, 161, 243, 209, 156, 162, 224, 219, 253, 249, 153, 155, 172, 1,
                ],
            ),
            (
                "41547452475632687683489977342365486797893454355756867843".try_into()?,
                vec![
                    131, 194, 247, 231, 163, 173, 225, 186, 194, 204, 202, 215, 213, 207, 147, 226,
                    197, 135, 146, 224, 236, 154, 165, 200, 198, 227, 6,
                ],
            ),
        ])
    }

    #[test]
    fn test_encode() -> Result<()> {
        for (value, bytes) in test_values()? {
            let encoded = NaturalBytesCoder::encode(&value)?;
            assert_eq!(encoded, bytes);
        }

        Ok(())
    }

    #[test]
    fn test_decode() -> Result<()> {
        for (value, bytes) in test_values()? {
            let decoded = NaturalBytesCoder::decode(&bytes)?;
            assert_eq!(value, decoded);
        }

        Ok(())
    }
}
