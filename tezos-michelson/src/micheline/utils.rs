use super::Micheline;
use crate::micheline::{
    literals::{Bytes, Int, String},
    PrimitiveApplication,
};

pub fn string<T>(value: T) -> Micheline
where
    T: std::convert::Into<String>,
{
    let string: String = value.into();
    string.into()
}

pub fn try_string<T, Error>(value: T) -> std::result::Result<Micheline, Error>
where
    T: std::convert::TryInto<String, Error = Error>,
{
    let string: String = value.try_into()?;
    Ok(string.into())
}

pub fn int<T>(value: T) -> Micheline
where
    T: std::convert::Into<Int>,
{
    let int: Int = value.into();
    int.into()
}

pub fn try_int<T, Error>(value: T) -> std::result::Result<Micheline, Error>
where
    T: std::convert::TryInto<Int, Error = Error>,
{
    let int: Int = value.try_into()?;
    Ok(int.into())
}

pub fn bytes<T>(value: T) -> Micheline
where
    T: std::convert::Into<Bytes>,
{
    let bytes: Bytes = value.into();
    bytes.into()
}

pub fn try_bytes<T, Error>(value: T) -> std::result::Result<Micheline, Error>
where
    T: std::convert::TryInto<Bytes, Error = Error>,
{
    let bytes: Bytes = value.try_into()?;
    Ok(bytes.into())
}

pub fn prim<T>(prim: T) -> Micheline
where
    T: std::convert::Into<std::string::String>,
{
    PrimitiveApplication::new(prim.into(), None, None).into()
}

pub fn prim_with_args<T>(prim: T, args: Vec<Micheline>) -> Micheline
where
    T: std::convert::Into<std::string::String>,
{
    PrimitiveApplication::new(prim.into(), Some(args), None).into()
}

pub fn prim_with_annots<T>(prim: T, annots: Vec<std::string::String>) -> Micheline
where
    T: std::convert::Into<std::string::String>,
{
    PrimitiveApplication::new(prim.into(), None, Some(annots)).into()
}

pub fn prim_with_args_and_annots<T>(
    prim: T,
    args: Vec<Micheline>,
    annots: Vec<std::string::String>,
) -> Micheline
where
    T: std::convert::Into<std::string::String>,
{
    PrimitiveApplication::new(prim.into(), Some(args), Some(annots)).into()
}

pub fn sequence(values: Vec<Micheline>) -> Micheline {
    values.into()
}
