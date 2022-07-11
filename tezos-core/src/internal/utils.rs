use crate::internal::consumable_list::ConsumableList;
use num_traits::ToPrimitive;

use crate::{Error, Result};

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

pub fn encode_i32(value: i32) -> [u8; 4] {
    let mut result = value.to_be_bytes();
    if value < 0 {
        for value in result.iter_mut() {
            if *value == 0u8 {
                *value = 255u8
            } else {
                break;
            }
        }
    }
    result
}

pub fn encode_i64(value: i64) -> [u8; 8] {
    let mut result = value.to_be_bytes();
    if value < 0 {
        for value in result.iter_mut() {
            if *value == 0u8 {
                *value = 255u8
            } else {
                break;
            }
        }
    }
    result
}
