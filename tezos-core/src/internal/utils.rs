use crate::{internal::consumable_list::ConsumableList, types::encoded::Encoded};
use crate::{Error, Result};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use num_traits::ToPrimitive;

use super::{coder::ConsumingDecoder, consumable_list::ConsumableBytes};

pub fn encode_string(value: &str) -> Vec<u8> {
    let bytes = value.as_bytes();
    encode_bytes(bytes)
}

pub fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len().to_u32().unwrap().to_be_bytes();

    [&length, bytes].concat()
}

pub fn decode_string<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<String> {
    let string_bytes = decode_bytes(bytes)?;
    Ok(String::from_utf8(string_bytes)?)
}

pub fn decode_bytes<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Vec<u8>> {
    let length_bytes: [u8; 4] = bytes
        .consume_until(4)?
        .try_into()
        .map_err(|_error| Error::InvalidBytes)?;
    let length = u32::from_be_bytes(length_bytes);

    Ok(bytes.consume_until(length as usize)?.into())
}

pub fn decode_annots<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Option<Vec<String>>> {
    let annots = decode_string(bytes)?;
    if annots.is_empty() {
        return Ok(None);
    }
    let annots = annots
        .split(' ')
        .map(|annot| annot.to_owned())
        .collect::<Vec<String>>();

    Ok(Some(annots))
}

pub fn encode_u16(value: u16) -> [u8; 2] {
    value.to_be_bytes()
}

pub fn decode_consuming_u16<CL: ConsumableList<u8>>(value: &mut CL) -> Result<u16> {
    let bytes = value.consume_until(2)?;

    Ok(u16::from_be_bytes(
        bytes.try_into().map_err(|_error| Error::InvalidBytes)?,
    ))
}

pub fn encode_i32(value: i32) -> [u8; 4] {
    value.to_be_bytes()
}

pub fn decode_consuming_i32<CL: ConsumableList<u8>>(value: &mut CL) -> Result<i32> {
    let bytes = value.consume_until(4)?;

    Ok(i32::from_be_bytes(
        bytes.try_into().map_err(|_error| Error::InvalidBytes)?,
    ))
}

pub fn encode_i64(value: i64) -> [u8; 8] {
    value.to_be_bytes()
}

pub fn decode_consuming_i64<CL: ConsumableList<u8>>(value: &mut CL) -> Result<i64> {
    let bytes = value.consume_until(8)?;

    Ok(i64::from_be_bytes(
        bytes.try_into().map_err(|_error| Error::InvalidBytes)?,
    ))
}

pub fn encode_bool(value: bool) -> [u8; 1] {
    if value {
        [255u8]
    } else {
        [0u8]
    }
}

pub fn decode_consuming_bool<CL: ConsumableList<u8>>(value: &mut CL) -> Result<bool> {
    let byte = value.consume_first()?;
    match byte {
        0 => Ok(false),
        255 => Ok(true),
        _ => Err(Error::InvalidBytes),
    }
}

pub fn encode_list<Item: Encoded>(list: &[Item]) -> Result<Vec<u8>> {
    let bytes = list
        .iter()
        .try_fold::<_, _, Result<_>>(Vec::<u8>::new(), |mut acc, item| {
            acc.append(&mut item.to_bytes()?);
            Ok(acc)
        })?;

    Ok(encode_bytes(&bytes))
}

pub fn decode_consuming_list<Item: Encoded, CL: ConsumableList<u8>>(
    value: &mut CL,
) -> Result<Vec<Item>>
where
    Item::Coder: ConsumingDecoder<Item, u8, Error>,
{
    let bytes = decode_bytes(value)?;
    let mut bytes = ConsumableBytes::new(&bytes);
    let mut result = Vec::<Item>::new();
    while !bytes.is_empty() {
        result.push(Item::from_consumable_bytes(&mut bytes)?);
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u16_coding() -> Result<()> {
        let encoded = encode_u16(1);
        let decoded = decode_consuming_u16(&mut ConsumableBytes::new(&encoded))?;

        assert_eq!(1, decoded);

        Ok(())
    }
}
