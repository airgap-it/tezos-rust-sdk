use tezos_core::internal::{
    coder::{ConsumingDecoder, Decoder, Encoder, IntegerBytesCoder},
    consumable_list::{ConsumableBytes, ConsumableList},
    utils,
};

use crate::{
    micheline::{
        literals::Literal, primitive_application::PrimitiveApplication, sequence::Sequence,
        Micheline,
    },
    michelson::Primitive,
    Error, Result,
};
use alloc::borrow::ToOwned;
use alloc::vec;
use alloc::vec::Vec;

pub struct MichelineBytesCoder;

impl Encoder<Micheline, Vec<u8>, Error> for MichelineBytesCoder {
    fn encode(value: &Micheline) -> Result<Vec<u8>> {
        match value {
            Micheline::Literal(value) => Self::encode(value),
            Micheline::PrimitiveApplication(value) => Self::encode(value),
            Micheline::Sequence(value) => Self::encode(value),
        }
    }
}

impl Decoder<Micheline, [u8], Error> for MichelineBytesCoder {
    fn decode(value: &[u8]) -> Result<Micheline> {
        let value = &mut ConsumableBytes::new(value);

        Self::decode_consuming(value)
    }
}

impl ConsumingDecoder<Micheline, u8, Error> for MichelineBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Micheline> {
        let byte = *value.inner_value().first().ok_or(Error::InvalidBytes)?;
        let tag = Tag::from_bytes(&[byte])?;
        match tag {
            Tag::Int | Tag::String | Tag::Bytes => {
                Self::decode_consuming(value).map(|literal: Literal| literal.into())
            }
            Tag::PrimNoArgsNoAnnots
            | Tag::PrimNoArgsSomeAnnots
            | Tag::Prim1ArgNoAnnots
            | Tag::Prim1ArgSomeAnnots
            | Tag::Prim2ArgsNoAnnots
            | Tag::Prim2ArgsSomeAnnots
            | Tag::PrimGeneric => {
                Self::decode_consuming(value).map(|prim: PrimitiveApplication| prim.into())
            }
            Tag::Sequence => {
                Self::decode_consuming(value).map(|sequence: Sequence| sequence.into())
            }
        }
    }
}

impl Encoder<Literal, Vec<u8>, Error> for MichelineBytesCoder {
    fn encode(value: &Literal) -> Result<Vec<u8>> {
        match value {
            Literal::Int(value) => Ok([Tag::Int.value(), &value.to_bytes()?].concat()),
            Literal::String(value) => {
                Ok([Tag::String.value(), &utils::encode_string(value.to_str())].concat())
            }
            Literal::Bytes(value) => {
                let bytes: Vec<u8> = value.into();
                Ok([Tag::Bytes.value(), &utils::encode_bytes(&bytes)].concat())
            }
        }
    }
}

impl Decoder<Literal, [u8], Error> for MichelineBytesCoder {
    fn decode(value: &[u8]) -> Result<Literal> {
        let value = &mut ConsumableBytes::new(value);

        Self::decode_consuming(value)
    }
}

impl ConsumingDecoder<Literal, u8, Error> for MichelineBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Literal> {
        let byte = value.consume_first()?;
        let tag = Tag::from_bytes(&[byte])?;
        match tag {
            Tag::Int => Ok(Literal::Int(IntegerBytesCoder::decode_consuming(value)?)),
            Tag::String => Ok(Literal::String(utils::decode_string(value)?.try_into()?)),
            Tag::Bytes => Ok(Literal::Bytes(utils::decode_bytes(value)?.into())),
            _ => Err(Error::InvalidBytes),
        }
    }
}

impl MichelineBytesCoder {
    fn encode_prim_no_args_no_annots(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;

        Ok([Tag::PrimNoArgsNoAnnots.value(), &[prim.tag()]].concat())
    }

    fn encode_prim_no_args_some_annots(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;

        Ok([
            Tag::PrimNoArgsSomeAnnots.value(),
            &[prim.tag()],
            &Self::encode_annots(value),
        ]
        .concat())
    }

    fn encode_prim_1_arg_no_annots(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;
        let arg_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .first()
            .map(|arg| arg.to_bytes())
            .ok_or(Error::InvalidPrimitiveApplication)??;

        Ok([Tag::Prim1ArgNoAnnots.value(), &[prim.tag()], &arg_bytes].concat())
    }

    fn encode_prim_1_arg_some_annots(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;
        let arg_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .first()
            .map(|arg| arg.to_bytes())
            .ok_or(Error::InvalidPrimitiveApplication)??;

        Ok([
            Tag::Prim1ArgSomeAnnots.value(),
            &[prim.tag()],
            &arg_bytes,
            &Self::encode_annots(value),
        ]
        .concat())
    }

    fn encode_prim_2_args_no_annots(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;
        let first_arg_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .first()
            .map(|arg| arg.to_bytes())
            .ok_or(Error::InvalidPrimitiveApplication)??;
        let second_arg_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .iter()
            .nth(1)
            .map(|arg| arg.to_bytes())
            .ok_or(Error::InvalidPrimitiveApplication)??;

        Ok([
            Tag::Prim2ArgsNoAnnots.value(),
            &[prim.tag()],
            &first_arg_bytes,
            &second_arg_bytes,
        ]
        .concat())
    }

    fn encode_prim_2_args_some_annots(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;
        let first_arg_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .first()
            .map(|arg| arg.to_bytes())
            .ok_or(Error::InvalidPrimitiveApplication)??;
        let second_arg_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .iter()
            .nth(1)
            .map(|arg| arg.to_bytes())
            .ok_or(Error::InvalidPrimitiveApplication)??;

        Ok([
            Tag::Prim2ArgsSomeAnnots.value(),
            &[prim.tag()],
            &first_arg_bytes,
            &second_arg_bytes,
            &Self::encode_annots(value),
        ]
        .concat())
    }

    fn encode_prim_generic(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let prim: Primitive = value.prim().try_into()?;
        let args_bytes = value
            .args()
            .as_ref()
            .ok_or(Error::InvalidPrimitiveApplication)?
            .iter()
            .map(|arg| arg.to_bytes())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Ok([
            Tag::PrimGeneric.value(),
            &[prim.tag()],
            &utils::encode_bytes(&args_bytes),
            &Self::encode_annots(value),
        ]
        .concat())
    }

    fn encode_annots(value: &PrimitiveApplication) -> Vec<u8> {
        let annots = value
            .annots()
            .as_ref()
            .map_or("".to_owned(), |annots| annots.join(" "));

        utils::encode_string(&annots)
    }

    fn decode_prim_no_args_no_annots<CL: ConsumableList<u8>>(
        bytes: &mut CL,
    ) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            None,
            None,
        ))
    }

    fn decode_prim_no_args_some_annots<CL: ConsumableList<u8>>(
        bytes: &mut CL,
    ) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;
        let annots = utils::decode_annots(bytes)?;

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            None,
            annots,
        ))
    }

    fn decode_prim_1_arg_no_annots<CL: ConsumableList<u8>>(
        bytes: &mut CL,
    ) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;
        let arg = Self::decode_consuming(bytes)?;

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            Some(vec![arg]),
            None,
        ))
    }

    fn decode_prim_1_arg_some_annots<CL: ConsumableList<u8>>(
        bytes: &mut CL,
    ) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;
        let arg = Self::decode_consuming(bytes)?;
        let annots = utils::decode_annots(bytes)?;

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            Some(vec![arg]),
            annots,
        ))
    }

    fn decode_prim_2_args_no_annots<CL: ConsumableList<u8>>(
        bytes: &mut CL,
    ) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;
        let first_arg = Self::decode_consuming(bytes)?;
        let second_arg = Self::decode_consuming(bytes)?;

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            Some(vec![first_arg, second_arg]),
            None,
        ))
    }

    fn decode_prim_2_args_some_annots<CL: ConsumableList<u8>>(
        bytes: &mut CL,
    ) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;
        let first_arg = Self::decode_consuming(bytes)?;
        let second_arg = Self::decode_consuming(bytes)?;
        let annots = utils::decode_annots(bytes)?;

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            Some(vec![first_arg, second_arg]),
            annots,
        ))
    }

    fn decode_prim_generic<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<PrimitiveApplication> {
        let prim: Primitive = bytes.consume_first()?.try_into()?;
        let decoded_bytes = utils::decode_bytes(bytes)?;
        let mut args_bytes = ConsumableBytes::new(&decoded_bytes);
        let mut args: Vec<Micheline> = Vec::new();
        while !args_bytes.is_empty() {
            let arg = Self::decode_consuming(&mut args_bytes)?;
            args.push(arg);
        }
        let annots = if !bytes.is_empty() {
            utils::decode_annots(bytes)?
        } else {
            None
        };

        Ok(PrimitiveApplication::new(
            prim.name().to_owned(),
            Some(args),
            annots,
        ))
    }
}

impl Encoder<PrimitiveApplication, Vec<u8>, Error> for MichelineBytesCoder {
    fn encode(value: &PrimitiveApplication) -> Result<Vec<u8>> {
        let has_annots = value
            .annots()
            .as_ref()
            .map_or(false, |annots| !annots.is_empty());
        let n_of_args = value.args().as_ref().map_or(0, |args| args.len());
        match (n_of_args, has_annots) {
            (0, true) => Self::encode_prim_no_args_some_annots(value),
            (0, false) => Self::encode_prim_no_args_no_annots(value),
            (1, true) => Self::encode_prim_1_arg_some_annots(value),
            (1, false) => Self::encode_prim_1_arg_no_annots(value),
            (2, true) => Self::encode_prim_2_args_some_annots(value),
            (2, false) => Self::encode_prim_2_args_no_annots(value),
            _ => Self::encode_prim_generic(value),
        }
    }
}

impl Decoder<PrimitiveApplication, [u8], Error> for MichelineBytesCoder {
    fn decode(value: &[u8]) -> Result<PrimitiveApplication> {
        let mut value = ConsumableBytes::new(value);
        Self::decode_consuming(&mut value)
    }
}

impl ConsumingDecoder<PrimitiveApplication, u8, Error> for MichelineBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<PrimitiveApplication> {
        let byte = value.consume_first()?;
        let tag = Tag::from_bytes(&[byte])?;

        match tag {
            Tag::PrimNoArgsNoAnnots => Self::decode_prim_no_args_no_annots(value),
            Tag::PrimNoArgsSomeAnnots => Self::decode_prim_no_args_some_annots(value),
            Tag::Prim1ArgNoAnnots => Self::decode_prim_1_arg_no_annots(value),
            Tag::Prim1ArgSomeAnnots => Self::decode_prim_1_arg_some_annots(value),
            Tag::Prim2ArgsNoAnnots => Self::decode_prim_2_args_no_annots(value),
            Tag::Prim2ArgsSomeAnnots => Self::decode_prim_2_args_some_annots(value),
            Tag::PrimGeneric => Self::decode_prim_generic(value),
            _ => Err(Error::InvalidBytes),
        }
    }
}

impl Encoder<Sequence, Vec<u8>, Error> for MichelineBytesCoder {
    fn encode(value: &Sequence) -> Result<Vec<u8>> {
        let bytes = value
            .values()
            .iter()
            .map(|item| item.to_bytes())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Ok([Tag::Sequence.value(), &utils::encode_bytes(&bytes)].concat())
    }
}

impl Decoder<Sequence, [u8], Error> for MichelineBytesCoder {
    fn decode(value: &[u8]) -> Result<Sequence> {
        let mut value = ConsumableBytes::new(value);
        Self::decode_consuming(&mut value)
    }
}

impl ConsumingDecoder<Sequence, u8, Error> for MichelineBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Sequence> {
        let byte = value.consume_first()?;
        let tag = Tag::from_bytes(&[byte])?;
        if tag != Tag::Sequence {
            return Err(Error::InvalidBytes);
        }
        let decoded_bytes = utils::decode_bytes(value)?;
        let mut bytes = ConsumableBytes::new(&decoded_bytes);
        let mut sequence = Vec::<Micheline>::new();
        while !bytes.is_empty() {
            let item = Self::decode_consuming(&mut bytes)?;
            sequence.push(item);
        }

        Ok(sequence.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tag {
    Int,
    String,
    Sequence,
    PrimNoArgsNoAnnots,
    PrimNoArgsSomeAnnots,
    Prim1ArgNoAnnots,
    Prim1ArgSomeAnnots,
    Prim2ArgsNoAnnots,
    Prim2ArgsSomeAnnots,
    PrimGeneric,
    Bytes,
}

impl Tag {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::Int => &[0],
            Self::String => &[1],
            Self::Sequence => &[2],
            Self::PrimNoArgsNoAnnots => &[3],
            Self::PrimNoArgsSomeAnnots => &[4],
            Self::Prim1ArgNoAnnots => &[5],
            Self::Prim1ArgSomeAnnots => &[6],
            Self::Prim2ArgsNoAnnots => &[7],
            Self::Prim2ArgsSomeAnnots => &[8],
            Self::PrimGeneric => &[9],
            Self::Bytes => &[10],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        match bytes {
            &[0] => Ok(Self::Int),
            &[1] => Ok(Self::String),
            &[2] => Ok(Self::Sequence),
            &[3] => Ok(Self::PrimNoArgsNoAnnots),
            &[4] => Ok(Self::PrimNoArgsSomeAnnots),
            &[5] => Ok(Self::Prim1ArgNoAnnots),
            &[6] => Ok(Self::Prim1ArgSomeAnnots),
            &[7] => Ok(Self::Prim2ArgsNoAnnots),
            &[8] => Ok(Self::Prim2ArgsSomeAnnots),
            &[9] => Ok(Self::PrimGeneric),
            &[10] => Ok(Self::Bytes),
            _ => Err(Error::InvalidBytes),
        }
    }
}

impl TryFrom<&[u8]> for Tag {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        Tag::from_bytes(value)
    }
}

#[cfg(test)]
mod test {
    use crate::micheline::{int, primitive_application, sequence, try_bytes, try_int, try_string};

    use super::*;

    #[test]
    fn encode() -> Result<()> {
        for (value, bytes) in int_values() {
            assert_eq!(bytes, &MichelineBytesCoder::encode(&value)?);
        }

        for (value, bytes) in string_values() {
            assert_eq!(bytes, &MichelineBytesCoder::encode(&value)?);
        }

        for (value, bytes) in bytes_values() {
            assert_eq!(bytes, &MichelineBytesCoder::encode(&value)?);
        }

        for (value, bytes) in primitive_application_values() {
            assert_eq!(bytes, &MichelineBytesCoder::encode(&value)?);
        }

        for (value, bytes) in sequence_values() {
            assert_eq!(bytes, &MichelineBytesCoder::encode(&value)?);
        }

        Ok(())
    }

    #[test]
    fn decode() -> Result<()> {
        for (value, bytes) in int_values() {
            assert_eq!(value, MichelineBytesCoder::decode(bytes)?);
        }

        for (value, bytes) in string_values() {
            assert_eq!(value, MichelineBytesCoder::decode(bytes)?);
        }

        for (value, bytes) in bytes_values() {
            assert_eq!(value, MichelineBytesCoder::decode(bytes)?);
        }

        for (value, bytes) in primitive_application_values() {
            assert_eq!(value, MichelineBytesCoder::decode(bytes)?);
        }

        for (value, bytes) in sequence_values() {
            assert_eq!(value, MichelineBytesCoder::decode(bytes)?);
        }

        Ok(())
    }

    fn int_values() -> Vec<(Micheline, &'static [u8])> {
        vec![
            (
                try_int("-41547452475632687683489977342365486797893454355756867843").unwrap(),
                &[
                    0, 195, 132, 239, 207, 199, 218, 194, 245, 132, 153, 149, 175, 171, 159, 167,
                    196, 139, 143, 164, 192, 217, 181, 202, 144, 141, 199, 13,
                ],
            ),
            (
                int(-54576326575686358562454576456764i128),
                &[
                    0, 252, 144, 211, 194, 230, 163, 185, 196, 192, 183, 251, 243, 179, 182, 216, 2,
                ],
            ),
            (
                int(-6852352674543413768i64),
                &[0, 200, 168, 221, 157, 248, 156, 185, 152, 190, 1],
            ),
            (
                int(-18756523543673i64),
                &[0, 249, 177, 226, 254, 226, 195, 8],
            ),
            (int(-128i8), &[0, 192, 2]),
            (int(-127i8), &[0, 255, 1]),
            (int(-64i8), &[0, 192, 1]),
            (int(-42i8), &[0, 106]),
            (int(-10i8), &[0, 74]),
            (int(-1i8), &[0, 65]),
            (int(0i8), &[0, 0]),
            (int(1i8), &[0, 1]),
            (int(10i8), &[0, 10]),
            (int(42i8), &[0, 42]),
            (int(64i8), &[0, 128, 1]),
            (int(127i8), &[0, 191, 1]),
            (int(128i16), &[0, 128, 2]),
            (
                int(18756523543673i64),
                &[0, 185, 177, 226, 254, 226, 195, 8],
            ),
            (
                int(6852352674543413768i64),
                &[0, 136, 168, 221, 157, 248, 156, 185, 152, 190, 1],
            ),
            (
                int(54576326575686358562454576456764i128),
                &[
                    0, 188, 144, 211, 194, 230, 163, 185, 196, 192, 183, 251, 243, 179, 182, 216, 2,
                ],
            ),
            (
                try_int("41547452475632687683489977342365486797893454355756867843").unwrap(),
                &[
                    0, 131, 132, 239, 207, 199, 218, 194, 245, 132, 153, 149, 175, 171, 159, 167,
                    196, 139, 143, 164, 192, 217, 181, 202, 144, 141, 199, 13,
                ],
            ),
        ]
    }

    fn string_values() -> Vec<(Micheline, &'static [u8])> {
        vec![
            (try_string("").unwrap(), &[1, 0, 0, 0, 0]),
            (try_string("a").unwrap(), &[1, 0, 0, 0, 1, 97]),
            (try_string("abc").unwrap(), &[1, 0, 0, 0, 3, 97, 98, 99]),
            (
                try_string("tz1ZsKMooGPJa5HoRT5DQV5j1RkRcSiypnYN").unwrap(),
                &[
                    1, 0, 0, 0, 36, 116, 122, 49, 90, 115, 75, 77, 111, 111, 71, 80, 74, 97, 53,
                    72, 111, 82, 84, 53, 68, 81, 86, 53, 106, 49, 82, 107, 82, 99, 83, 105, 121,
                    112, 110, 89, 78,
                ],
            ),
        ]
    }

    fn bytes_values() -> Vec<(Micheline, &'static [u8])> {
        vec![
            (try_bytes("0x").unwrap(), &[10, 0, 0, 0, 0]),
            (try_bytes("0x00").unwrap(), &[10, 0, 0, 0, 1, 0]),
            (
                try_bytes("0x9434dc98").unwrap(),
                &[10, 0, 0, 0, 4, 148, 52, 220, 152],
            ),
            (
                try_bytes("0x7b1ea2cb").unwrap(),
                &[10, 0, 0, 0, 4, 123, 30, 162, 203],
            ),
            (
                try_bytes("0xe40476d7").unwrap(),
                &[10, 0, 0, 0, 4, 228, 4, 118, 215],
            ),
            (
                try_bytes("0xc47320abdd31").unwrap(),
                &[10, 0, 0, 0, 6, 196, 115, 32, 171, 221, 49],
            ),
            (
                try_bytes("0x5786dac9eaf4").unwrap(),
                &[10, 0, 0, 0, 6, 87, 134, 218, 201, 234, 244],
            ),
        ]
    }

    fn primitive_application_values() -> Vec<(Micheline, &'static [u8])> {
        vec![
            (
                primitive_application("parameter")
                    .with_args(vec![primitive_application("unit").into()])
                    .into(),
                &[5, 0, 3, 108],
            ),
            (
                primitive_application("parameter")
                    .with_args(vec![primitive_application("unit").into()])
                    .with_annots(vec!["%parameter".into()])
                    .into(),
                &[
                    6, 0, 3, 108, 0, 0, 0, 10, 37, 112, 97, 114, 97, 109, 101, 116, 101, 114,
                ],
            ),
            (
                primitive_application("parameter")
                    .with_args(vec![primitive_application("unit").into()])
                    .with_annots(vec!["%annot1".into(), "@annot2".into()])
                    .into(),
                &[
                    6, 0, 3, 108, 0, 0, 0, 15, 37, 97, 110, 110, 111, 116, 49, 32, 64, 97, 110,
                    110, 111, 116, 50,
                ],
            ),
            (
                primitive_application("parameter")
                    .with_args(vec![primitive_application("unit").into()])
                    .with_annots(vec![
                        "@annot1".into(),
                        ":annot2".into(),
                        "$annot3".into(),
                        "&annot4".into(),
                        "%annot5".into(),
                        "!annot6".into(),
                        "?annot7".into(),
                    ])
                    .into(),
                &[
                    6, 0, 3, 108, 0, 0, 0, 55, 64, 97, 110, 110, 111, 116, 49, 32, 58, 97, 110,
                    110, 111, 116, 50, 32, 36, 97, 110, 110, 111, 116, 51, 32, 38, 97, 110, 110,
                    111, 116, 52, 32, 37, 97, 110, 110, 111, 116, 53, 32, 33, 97, 110, 110, 111,
                    116, 54, 32, 63, 97, 110, 110, 111, 116, 55,
                ],
            ),
            (
                primitive_application("Pair")
                    .with_args(vec![
                        primitive_application("Unit").into(),
                        primitive_application("Unit").into(),
                        primitive_application("Unit").into(),
                    ])
                    .into(),
                &[9, 7, 0, 0, 0, 6, 3, 11, 3, 11, 3, 11, 0, 0, 0, 0],
            ),
        ]
    }

    fn sequence_values() -> Vec<(Micheline, &'static [u8])> {
        vec![
            (sequence::<Vec<Micheline>, _>(vec![]), &[2, 0, 0, 0, 0]),
            (vec![int(0)].into(), &[2, 0, 0, 0, 2, 0, 0]),
            (
                vec![int(0), try_string("abc").unwrap()].into(),
                &[2, 0, 0, 0, 10, 0, 0, 1, 0, 0, 0, 3, 97, 98, 99],
            ),
        ]
    }
}
