macro_rules! make_all_data {
    (
        $(custom_cases: {
            $($enum_case_name:ident($enum_case_type:ty),)+
        })?,
        $(
            (
                $name:ident, $mod_name:ident, $tag:literal
                $(, ($field_name:ident: $field_type:ty))*
                $(, optional: ($opt_field_name:ident: $opt_field_type:ty))*
                $(, boxed: ($boxed_field_name:ident: $boxed_field_type:ty))*
                $(, vec: ($vec_field_name:ident: $vec_field_type:ty))*
            ),
        )+
    ) => {
        use tezos_core::internal::traits::InnerValueRef;
        use crate::{michelson::{Michelson, Literal}, Result, Error, micheline::{Micheline, primitive_application::PrimitiveApplication}, common::macros::make_primitive_enum};
        pub use self::{
            instructions::Instruction,
            $($mod_name::{$name, $mod_name},)*
        };

        /// Tezos Michelson data types as defined in [the documentation](https://tezos.gitlab.io/active/michelson.html#full-grammar).
        ///
        /// See also: [Michelson Reference](https://tezos.gitlab.io/michelson-reference/).
        #[derive(Debug, Clone, PartialEq)]
        pub enum Data {
            $($($enum_case_name($enum_case_type),)*)+
            $($name($name),)*
        }

        $(
            $(
                impl TryFrom<Data> for $enum_case_type {
                    type Error = Error;

                    fn try_from(value: Data) -> Result<$enum_case_type> {
                        if let Data::$enum_case_name(value) = value {
                            return Ok(value);
                        }
                        Err(Error::InvalidMichelsonData)
                    }
                }

                impl From<$enum_case_type> for Data {
                    fn from(value: $enum_case_type) -> Self {
                        Self::$enum_case_name(value)
                    }
                }
            )+
        )?

        $(
            $(
                impl InnerValueRef<$enum_case_name> for Data {
                    fn inner_value_ref(&self) -> Option<&$enum_case_type> {
                        if let Data::$enum_case_name(value) = self {
                            return Option::Some(value);
                        }
                        Option::None
                    }
                }
            )+
        )?

        impl From<Data> for Micheline {
            fn from(value: Data) -> Self {
                match value {
                    $(
                        Data::$name(value) => value.into(),
                    )*
                    $(
                        $(
                            Data::$enum_case_name(value) => value.into(),
                        )*
                    )+
                }
            }
        }

        impl From<&Data> for Micheline {
            fn from(value: &Data) -> Self {
                match value {
                    $(
                        Data::$name(value) => value.into(),
                    )*
                    $(
                        $(
                            Data::$enum_case_name(value) => value.into(),
                        )*
                    )+
                }
            }
        }

        impl From<Data> for Michelson {
            fn from(value: Data) -> Self {
                Self::Data(value)
            }
        }

        impl TryFrom<Michelson> for Data {
            type Error = Error;

            fn try_from(value: Michelson) -> Result<Self> {
                if let Michelson::Data(value) = value {
                    return Ok(value);
                }
                Err(Error::InvalidMichelson)
            }
        }

        impl TryFrom<Micheline> for Data {
            type Error = Error;

            fn try_from(value: Micheline) -> Result<Self> {
                match value {
                    Micheline::Literal(value) => Ok(value.into()),
                    Micheline::PrimitiveApplication(value) => value.try_into(),
                    Micheline::Sequence(_) => Ok(Data::Sequence(value.try_into()?)),
                }
            }
        }

        impl TryFrom<PrimitiveApplication> for Data {
            type Error = Error;

            fn try_from(value: PrimitiveApplication) -> Result<Self> {
                match value.prim() {
                    $(stringify!($name) => Ok(Data::$name(value.try_into()?)),)*
                    _ => Ok(Data::Instruction(value.try_into()?)),
                }
            }
        }

        make_primitive_enum!($($name, $name, $tag)+);

        $(
            make_data!(
                $name, $mod_name, $tag
                $(, ($field_name: $field_type))*
                $(, optional: ($opt_field_name: $opt_field_type))*
                $(, boxed: ($boxed_field_name: $boxed_field_type))*
                $(, vec: ($vec_field_name: $vec_field_type))*
            );
        )+
    };
}

macro_rules! make_data {
    ($name:ident, $mod_name:ident, $tag:literal) => {
        mod $mod_name {
            use tezos_core::internal::traits::InnerValueRef;
            use crate::{
                michelson::{PrimType, Michelson},
                micheline::{Micheline, primitive_application::PrimitiveApplication},
                Error, Result,
            };
            use super::{Data, Primitive};
            #[allow(unused_imports)]
            use alloc::vec::Vec;
            use alloc::vec;

            #[derive(Debug, Clone, PartialEq)]
            pub struct $name;

            impl PrimType for $name {
                fn prim_value() -> crate::michelson::Primitive {
                    crate::michelson::Primitive::Data(Primitive::$name)
                }
            }

            impl From<$name> for Data {
                fn from(value: $name) -> Self {
                    Data::$name(value)
                }
            }

            impl TryFrom<Data> for $name {
                type Error = Error;

                fn try_from(value: Data) -> Result<Self> {
                    if let Data::$name(value) = value {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonData)
                }
            }

            impl From<$name> for Michelson {
                fn from(value: $name) -> Self {
                    Self::Data(value.into())
                }
            }

            impl TryFrom<Michelson> for $name {
                type Error = Error;

                fn try_from(value: Michelson) -> Result<Self> {
                    let data: Data = value.try_into()?;

                    if let Data::$name(value) = data {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonData)
                }
            }

            impl From<$name> for Micheline {
                fn from(_: $name) -> Self {
                    let primitive_application = PrimitiveApplication::new($name::prim_value().name().into(), Option::None, Option::None);

                    primitive_application.into()
                }
            }

            impl From<&$name> for Micheline {
                fn from(_: &$name) -> Self {
                    let primitive_application = PrimitiveApplication::new($name::prim_value().name().into(), Option::None, Option::None);

                    primitive_application.into()
                }
            }

            impl TryFrom<PrimitiveApplication> for $name {
                type Error = Error;

                #[allow(unused)]
                fn try_from(value: PrimitiveApplication) -> Result<Self> {
                    if value.prim() != Self::prim_value().name() {
                        return Err(Error::InvalidPrimitiveApplication);
                    }
                    let mut args = value.into_args().unwrap_or(vec![]);
                    Ok($name)
                }
            }

            impl InnerValueRef<$name> for Data {
                fn inner_value_ref(&self) -> Option<&$name> {
                    if let Data::$name(value) = self {
                        return Option::Some(value);
                    }
                    Option::None
                }
            }

            pub fn $mod_name<Output>() -> Output where Output: From<$name> {
                $name.into()
            }
        }
    };
    (
        $name:ident, $mod_name:ident, $tag:literal
        $(, ($field_name:ident: $field_type:ty))*
        $(, optional: ($opt_field_name:ident: $opt_field_type:ty))*
        $(, boxed: ($boxed_field_name:ident: $boxed_field_type:ty))*
        $(, vec: ($vec_field_name:ident: $vec_field_type:ty))*
    ) => {
        mod $mod_name {
            use tezos_core::internal::traits::InnerValueRef;
            use crate::{
                michelson::{PrimType, Michelson, Data, data::Primitive},
                micheline::{Micheline, primitive_application::PrimitiveApplication},
                Error, Result,
            };
            use alloc::vec::Vec;
            use alloc::vec;
            #[allow(unused_imports)]
            use alloc::boxed::Box;

            #[derive(Debug, Clone, PartialEq)]
            pub struct $name {
                $(
                    pub $field_name: $field_type,
                )*
                $(
                    pub $opt_field_name: Option<$opt_field_type>,
                )*
                $(
                    pub $boxed_field_name: Box<$boxed_field_type>,
                )*
                $(
                    pub $vec_field_name: Vec<$vec_field_type>,
                )*
            }

            impl $name {
                pub fn new($($field_name: $field_type,)* $($opt_field_name: Option<$opt_field_type>,)* $($boxed_field_name: $boxed_field_type,)* $($vec_field_name: Vec<$vec_field_type>,)*) -> Self {
                    Self {
                        $($field_name,)*
                        $($opt_field_name,)*
                        $($boxed_field_name: Box::new($boxed_field_name),)*
                        $($vec_field_name,)*
                    }
                }
            }

            impl PrimType for $name {
                fn prim_value() -> crate::michelson::Primitive {
                    Primitive::$name.into()
                }
            }

            impl InnerValueRef<$name> for Data {
                fn inner_value_ref(&self) -> Option<&$name> {
                    if let Data::$name(value) = self {
                        return Option::Some(value);
                    }
                    Option::None
                }
            }

            impl From<$name> for Data {
                fn from(value: $name) -> Self {
                    Data::$name(value)
                }
            }

            impl TryFrom<Data> for $name {
                type Error = Error;

                fn try_from(value: Data) -> Result<Self> {
                    if let Data::$name(value) = value {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonData)
                }
            }

            impl From<$name> for Michelson {
                fn from(value: $name) -> Self {
                    Self::Data(value.into())
                }
            }

            impl TryFrom<Michelson> for $name {
                type Error = Error;

                fn try_from(value: Michelson) -> Result<Self> {
                    let data: Data = value.try_into()?;

                    if let Data::$name(value) = data {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonData)
                }
            }

            impl From<$name> for Micheline {

                fn from(value: $name) -> Self {
                    let mut args: Vec<Micheline> = vec![];
                    $(
                        args.push(value.$field_name.into());
                    )*
                    $(
                        if let Some(value) = value.$opt_field_name {
                            args.push(value.into());
                        }
                    )*
                    $(
                        args.push((*value.$boxed_field_name).into());
                    )*
                    $(
                        let mut values = value.$vec_field_name.into_iter().map(|value| value.into()).collect::<Vec<Micheline>>();
                        args.append(&mut values);
                    )*

                    let primitive_application = PrimitiveApplication::new($name::prim_value().name().into(), Some(args), Option::None);

                    primitive_application.into()
                }
            }

            impl From<&$name> for Micheline {

                fn from(value: &$name) -> Self {
                    let mut args: Vec<Micheline> = vec![];
                    $(
                        args.push(value.$field_name.into());
                    )*
                    $(
                        if let Some(value) = value.$opt_field_name {
                            args.push(value.into());
                        }
                    )*
                    $(
                        args.push((&*value.$boxed_field_name).into());
                    )*
                    $(
                        let mut values = value.$vec_field_name.iter().map(|value| value.into()).collect::<Vec<Micheline>>();
                        args.append(&mut values);
                    )*

                    let primitive_application = PrimitiveApplication::new($name::prim_value().name().into(), Some(args), Option::None);

                    primitive_application.into()
                }
            }

            impl TryFrom<PrimitiveApplication> for $name {
                type Error = Error;

                #[allow(unused)]
                fn try_from(value: PrimitiveApplication) -> Result<Self> {
                    if value.prim() != Self::prim_value().name() {
                        return Err(Error::InvalidPrimitiveApplication);
                    }
                    let mut args = value.into_args().unwrap_or(vec![]);
                    Ok(Self {
                        $(
                            $field_name: if !args.is_empty() { args.remove(0).try_into()? } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        $(
                            $opt_field_name: if !args.is_empty() { Some(args.remove(0).try_into()?) } else { None },
                        )*
                        $(
                            $boxed_field_name: if !args.is_empty() { Box::new(args.remove(0).try_into()?) } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        $(
                            $vec_field_name: if !args.is_empty() { args.into_iter().map(|value| value.try_into()).collect::<Result<Vec<_>>>()? } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                    })
                }
            }

            pub fn $mod_name<Output>($($field_name: $field_type,)* $($opt_field_name: Option<$opt_field_type>,)* $($boxed_field_name: $boxed_field_type,)* $($vec_field_name: Vec<$vec_field_type>,)*) -> Output where Output: From<$name> {
                $name::new(
                    $($field_name, )*
                    $($opt_field_name, )*
                    $($boxed_field_name,)*
                    $($vec_field_name,)*
                ).into()
            }
        }
    };
}

pub(crate) use {make_all_data, make_data};
