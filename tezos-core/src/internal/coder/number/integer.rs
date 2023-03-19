use num_bigint::{BigInt, BigUint, ToBigUint};
use num_traits::{Signed, ToPrimitive, Zero};

use super::natural::NaturalBytesCoder;
use crate::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::{ConsumableBytes, ConsumableList},
    },
    types::number::Int,
    Error, Result,
};
use alloc::{vec, vec::Vec};

pub struct IntegerBytesCoder;

impl Encoder<Int, Vec<u8>, Error> for IntegerBytesCoder {
    fn encode(value: &Int) -> Result<Vec<u8>> {
        let value: BigInt = value.to_integer()?;
        let abs = value.abs().to_biguint().unwrap();

        let byte = &abs & BigUint::from(0b0011_1111u8);
        let next_value = abs >> 6u8;

        let sequence_mask = if next_value == BigUint::zero() {
            BigUint::from(0b0000_0000u8)
        } else {
            BigUint::from(0b1000_0000u8)
        };
        let sign_mask = if value < BigInt::zero() {
            BigUint::from(0b0100_0000u8)
        } else {
            BigUint::from(0b0000_0000u8)
        };
        let encoded_byte = (byte | sequence_mask | sign_mask).to_u8().unwrap();

        let next_value_encoded = if next_value > BigUint::zero() {
            NaturalBytesCoder::encode_unsigned(next_value)
        } else {
            vec![]
        };

        Ok([vec![encoded_byte], next_value_encoded].concat())
    }
}

impl Decoder<Int, Vec<u8>, Error> for IntegerBytesCoder {
    fn decode(value: &Vec<u8>) -> Result<Int> {
        let value = &mut ConsumableBytes::new(value);

        Self::decode_consuming(value)
    }
}

impl ConsumingDecoder<Int, u8, Error> for IntegerBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Int> {
        let byte = value.consume_first()?;
        let part = BigInt::from(byte & 0b0011_1111u8);
        let sign = if (byte & 0b0100_0000u8) == 0b0100_0000u8 {
            -1i8
        } else {
            1i8
        };
        let has_next = (byte & 0b1000_0000u8) == 0b1000_0000u8;
        let abs = if has_next {
            let decoded: BigInt = NaturalBytesCoder::decode_consuming(value)?
                .to_biguint()
                .unwrap()
                .into();
            part + (decoded << 6u8)
        } else {
            part
        };
        let result: BigInt = abs * sign;

        return Ok(result.into());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_values() -> Result<Vec<(Int, Vec<u8>)>> {
        Ok(vec![
            (
                "-41547452475632687683489977342365486797893454355756867843".try_into()?,
                vec![
                    195, 132, 239, 207, 199, 218, 194, 245, 132, 153, 149, 175, 171, 159, 167, 196,
                    139, 143, 164, 192, 217, 181, 202, 144, 141, 199, 13,
                ],
            ),
            (
                "-54576326575686358562454576456764".try_into()?,
                vec![
                    252, 144, 211, 194, 230, 163, 185, 196, 192, 183, 251, 243, 179, 182, 216, 2,
                ],
            ),
            (
                (-6852352674543413768i64).into(),
                vec![200, 168, 221, 157, 248, 156, 185, 152, 190, 1],
            ),
            (
                (-18756523543673i64).into(),
                vec![249, 177, 226, 254, 226, 195, 8],
            ),
            ((-128i8).into(), vec![192, 2]),
            ((-127i8).into(), vec![255, 1]),
            ((-64i8).into(), vec![192, 1]),
            ((-42i8).into(), vec![106]),
            ((-10i8).into(), vec![74]),
            ((-1i8).into(), vec![65]),
            ((0i8).into(), vec![0]),
            ((1i8).into(), vec![1]),
            ((10i8).into(), vec![10]),
            ((42i8).into(), vec![42]),
            ((64i8).into(), vec![128, 1]),
            ((127i8).into(), vec![191, 1]),
            ((128i32).into(), vec![128, 2]),
            (
                (18756523543673i64).into(),
                vec![185, 177, 226, 254, 226, 195, 8],
            ),
            (
                (6852352674543413768i64).into(),
                vec![136, 168, 221, 157, 248, 156, 185, 152, 190, 1],
            ),
            (
                "54576326575686358562454576456764".try_into()?,
                vec![
                    188, 144, 211, 194, 230, 163, 185, 196, 192, 183, 251, 243, 179, 182, 216, 2,
                ],
            ),
            (
                "41547452475632687683489977342365486797893454355756867843".try_into()?,
                vec![
                    131, 132, 239, 207, 199, 218, 194, 245, 132, 153, 149, 175, 171, 159, 167, 196,
                    139, 143, 164, 192, 217, 181, 202, 144, 141, 199, 13,
                ],
            ),
        ])
    }

    #[test]
    fn test_encode() -> Result<()> {
        for (value, bytes) in test_values()? {
            let encoded = IntegerBytesCoder::encode(&value)?;
            assert_eq!(encoded, bytes);
        }

        Ok(())
    }

    #[test]
    fn test_decode() -> Result<()> {
        for (value, bytes) in test_values()? {
            let decoded = IntegerBytesCoder::decode(&bytes)?;
            assert_eq!(value, decoded);
        }

        Ok(())
    }
}
