use super::{sequence::Sequence, Micheline};
use crate::micheline::{
    literals::{Bytes, Int, String},
    PrimitiveApplication,
};

pub fn string<T, Output>(value: T) -> Output
where
    T: std::convert::Into<String>,
    Output: From<String>,
{
    let string: String = value.into();
    string.into()
}

pub fn try_string<T, Output, Error>(value: T) -> std::result::Result<Output, Error>
where
    T: std::convert::TryInto<String, Error = Error>,
    Output: From<String>,
{
    let string: String = value.try_into()?;
    Ok(string.into())
}

pub fn int<T, Output>(value: T) -> Output
where
    T: std::convert::Into<Int>,
    Output: From<Int>,
{
    let int: Int = value.into();
    int.into()
}

pub fn try_int<T, Output, Error>(value: T) -> std::result::Result<Output, Error>
where
    T: std::convert::TryInto<Int, Error = Error>,
    Output: From<Int>,
{
    let int: Int = value.try_into()?;
    Ok(int.into())
}

pub fn bytes<T, Output>(value: T) -> Output
where
    T: std::convert::Into<Bytes>,
    Output: From<Bytes>,
{
    let bytes: Bytes = value.into();
    bytes.into()
}

pub fn try_bytes<T, Output, Error>(value: T) -> std::result::Result<Output, Error>
where
    T: std::convert::TryInto<Bytes, Error = Error>,
    Output: From<Bytes>,
{
    let bytes: Bytes = value.try_into()?;
    Ok(bytes.into())
}

pub fn primitive_application<T>(prim: T) -> PrimitiveApplication
where
    T: std::convert::Into<std::string::String>,
{
    PrimitiveApplication::new(prim.into(), None, None)
}

pub fn sequence<T, Output>(values: Vec<Micheline>) -> Output
where
    T: std::convert::Into<Vec<Micheline>>,
    Output: From<Sequence>,
{
    let sequence: Sequence = values.into();
    sequence.into()
}
