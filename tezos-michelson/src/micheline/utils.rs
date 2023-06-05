use super::{sequence::Sequence, Micheline};
use crate::micheline::{
    literals::{Bytes, Int, String},
    PrimitiveApplication,
};
use alloc::vec::Vec;

/// Utility function to create a tezos [String] and convert it to
/// a type inferred by the context at call location.
///
/// ```rust
/// use tezos_michelson::{micheline::{Micheline, try_string, literals::String as TezosString}, michelson::Michelson, Result};
///
/// fn create_tezos_string() -> Result<()> {
///     let value: TezosString = try_string("string")?;
///     let value: Micheline = try_string("string")?;
///     let value: Michelson = try_string("string")?;
///
///     Ok(())
/// }
/// ```
pub fn try_string<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<String, Error = Error>,
    Output: From<String>,
{
    let string: String = value.try_into()?;
    Ok(string.into())
}

/// Utility function to create a tezos [Int] and convert it to
/// a type inferred by the context at call location.
///
/// ```rust
/// use tezos_michelson::{micheline::{Micheline, int, literals::Int}, michelson::Michelson};
///
/// let value: Int = int(10);
/// let value: Micheline = int(10);
/// let value: Michelson = int(10);
/// ```
pub fn int<T, Output>(value: T) -> Output
where
    T: core::convert::Into<Int>,
    Output: From<Int>,
{
    let int: Int = value.into();
    int.into()
}

/// Utility function to create a tezos [Int] and convert it to
/// a type inferred by the context at call location.
///
/// ```rust
/// use tezos_michelson::{micheline::{Micheline, try_int, literals::Int}, michelson::Michelson, Result};
///
/// fn create_tezos_int() -> Result<()> {
///     let value: Int = try_int("10")?;
///     let value: Micheline = try_int("10")?;
///     let value: Michelson = try_int("10")?;
///
///     Ok(())
/// }
/// ```
pub fn try_int<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<Int, Error = Error>,
    Output: From<Int>,
{
    let int: Int = value.try_into()?;
    Ok(int.into())
}

/// Utility function to create a tezos [Bytes] and convert it to
/// a type inferred by the context at call location.
///
/// ```rust
/// use tezos_michelson::{micheline::{Micheline, bytes, literals::Bytes}, michelson::Michelson};
///
/// let value: Bytes = bytes(vec![10u8]);
/// let value: Micheline = bytes(vec![10u8]);
/// let value: Michelson = bytes(vec![10u8]);
/// ```
pub fn bytes<T, Output>(value: T) -> Output
where
    T: core::convert::Into<Bytes>,
    Output: From<Bytes>,
{
    let bytes: Bytes = value.into();
    bytes.into()
}

/// Utility function to create a tezos [Bytes] and convert it to
/// a type inferred by the context at call location.
///
/// ```rust
/// use tezos_michelson::{micheline::{Micheline, try_bytes, literals::Bytes}, michelson::Michelson, Result};
///
/// fn create_tezos_bytes() -> Result<()> {
///     let value: Bytes = try_bytes("0x0a")?;
///     let value: Micheline = try_bytes("0x0a")?;
///     let value: Michelson = try_bytes("0x0a")?;
///
///     Ok(())
/// }
/// ```
pub fn try_bytes<T, Output, Error>(value: T) -> core::result::Result<Output, Error>
where
    T: core::convert::TryInto<Bytes, Error = Error>,
    Output: From<Bytes>,
{
    let bytes: Bytes = value.try_into()?;
    Ok(bytes.into())
}

/// Utility function to create a tezos [PrimitiveApplication].
///
/// ```rust
/// use tezos_michelson::{micheline::{primitive_application, primitive_application::PrimitiveApplication}, michelson::ComparableTypePrimitive};
///
/// let value = primitive_application(ComparableTypePrimitive::Nat);
/// ```
pub fn primitive_application<T>(prim: T) -> PrimitiveApplication
where
    T: core::convert::Into<alloc::string::String>,
{
    PrimitiveApplication::new(prim.into(), None, None)
}

/// Utility function to create a tezos [Sequence] and convert it to
/// a type inferred by the context at call location.
///
/// ```rust
/// use tezos_michelson::{micheline::{Micheline, sequence, int, bytes, sequence::Sequence}};
///
/// let value: Sequence = sequence(vec![int(10), bytes(vec![10u8])]);
/// let value: Micheline = sequence(vec![int(10), bytes(vec![10u8])]);
pub fn sequence<T, Output>(values: T) -> Output
where
    T: core::convert::Into<Vec<Micheline>>,
    Output: From<Sequence>,
{
    let values: Vec<Micheline> = values.into();
    let sequence: Sequence = values.into();
    sequence.into()
}
