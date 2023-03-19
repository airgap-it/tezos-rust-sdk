macro_rules! make_types {
    (
        $(type_enum: $enum_case_name:ident($enum_case_type:ty),)?
        [$($type_impl:tt)*],
        conversion_fallback: $fallback:ident,
        $(
            (
                $name:ident, $code:ident, $tag:literal
                $(, super_enum: $super_enum_type:ty, $super_enum_case:ident)?
                $(, ($field_name:ident: $field_type:ty))*
                $(, boxed: ($boxed_field_name:ident: $boxed_field_type:ty))*
                $(, vec: ($vec_field_name:ident: $vec_field_type:ty))*
            ),
        )+
    ) => {
        use crate::{micheline::{Micheline, primitive_application::PrimitiveApplication}, michelson::metadata::TypeFieldMetadata, common::macros::make_primitive_enum};
        pub use self::{
            $($code::{$name, $code},)*
        };

        /// Tezos Michelson types as defined in [the documentation](https://tezos.gitlab.io/active/michelson.html#full-grammar).
        ///
        /// See also: [Michelson Reference](https://tezos.gitlab.io/michelson-reference/).
        #[derive(Debug, Clone, PartialEq)]
        pub enum Type {
            $($enum_case_name($enum_case_type),)?
            $($name($name),)*
        }

        impl Type {
            $(
                $type_impl
            )*

            pub fn metadata(&self) -> &TypeFieldMetadata {
                match self {
                    $(
                        Type::$enum_case_name(value) => value.metadata(),
                    )?
                    $(
                        Type::$name(value) => value.metadata(),
                    )*
                }
            }
        }

        $(
            use tezos_core::internal::traits::InnerValueRef;

            impl InnerValueRef<$enum_case_type> for Type {
                fn inner_value_ref(&self) -> core::option::Option<&$enum_case_type> {
                    if let Type::$enum_case_name(value) = self {
                        return Some(value);
                    }

                    None
                }
            }
        )?

        impl From<Type> for Micheline {
            fn from(value: Type) -> Self {
                match value {
                    $(
                        Type::$enum_case_name(value) => value.into(),
                    )?
                    $(
                        Type::$name(value) => value.into(),
                    )*
                }
            }
        }

        impl From<&Type> for Micheline {
            fn from(value: &Type) -> Self {
                match value {
                    $(
                        Type::$enum_case_name(value) => value.into(),
                    )?
                    $(
                        Type::$name(value) => value.into(),
                    )*
                }
            }
        }

        impl TryFrom<Micheline> for Type {
            type Error = Error;

            fn try_from(value: Micheline) -> Result<Self> {
                let primitive_application: PrimitiveApplication = value.try_into()?;

                primitive_application.try_into()
            }
        }

        impl TryFrom<PrimitiveApplication> for Type {
            type Error = Error;

            fn try_from(value: PrimitiveApplication) -> Result<Self> {
                match value.prim() {
                    $(stringify!($code) => Ok(Type::$name(value.try_into()?)),)*
                    _ => Self::$fallback(value),
                }
            }
        }

        make_primitive_enum!($($name, $code, $tag)+);

        $(
            make_type!(
                $name, $code, $tag
                $(, super_enum: $super_enum_type, $super_enum_case)?
                $(, ($field_name, $field_type))*
                $(, boxed: ($boxed_field_name: $boxed_field_type))*
                $(, vec: ($vec_field_name: $vec_field_type))*
            );
        )+
    };
}

macro_rules! make_type {
    (
        $name:ident, $code:ident, $tag:literal
        $(, super_enum: $super_enum_type:ty, $super_enum_case:ident)?
        $(, ($field_name:ident, $field_type:ty))*
        $(, boxed: ($boxed_field_name:ident: $boxed_field_type:ty))*
        $(, vec: ($vec_field_name:ident: $vec_field_type:ty))*
    ) => {
        mod $code {
            use tezos_core::internal::traits::InnerValueRef;
            use crate::{
                micheline::{Micheline, primitive_application::PrimitiveApplication},
                michelson::{Annotation, metadata::TypeFieldMetadata, PrimType, Michelson},
                Error, Result,
            };
            use super::{Type, Primitive};
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
                    pub $boxed_field_name: Box<$boxed_field_type>,
                )*
                $(
                    pub $vec_field_name: Vec<$vec_field_type>,
                )*
                pub(crate) metadata: TypeFieldMetadata,
            }

            impl $name {
                pub fn metadata(&self) -> &TypeFieldMetadata {
                    &self.metadata
                }

                pub fn annotations(&self) -> Vec<&Annotation> {
                    self.metadata.annotations()
                }

                pub fn new($($field_name: $field_type,)* $($boxed_field_name: $boxed_field_type,)* $($vec_field_name: Vec<$vec_field_type>,)* metadata: core::option::Option<TypeFieldMetadata>) -> Self {
                    Self {
                        $($field_name,)*
                        $($boxed_field_name: Box::new($boxed_field_name),)*
                        $($vec_field_name,)*
                        metadata: metadata.unwrap_or_default()
                    }
                }

                pub fn with_type_annotation<Output>(mut self, annotation: alloc::string::String) -> Output where Output: From<$name> {
                    self.metadata = self.metadata.with_type_name(annotation);

                    self.into()
                }

                pub fn with_field_annotation<Output>(mut self, annotation: alloc::string::String) -> Output where Output: From<$name> {
                    self.metadata = self.metadata.with_field_name(annotation);

                    self.into()
                }
            }

            impl PrimType for $name {
                fn prim_value() -> crate::michelson::Primitive {
                    Primitive::$name.into()
                }
            }

            impl InnerValueRef<$name> for Type {
                fn inner_value_ref(&self) -> core::option::Option<&$name> {
                    if let Type::$name(value) = self {
                        return Some(value);
                    }

                    None
                }
            }

            impl From<$name> for Michelson {
                fn from(value: $name) -> Self {
                    let value: Type = value.into();
                    Self::Type(value.into())
                }
            }

            impl From<$name> for Type {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }

            $(
                impl From<$name> for $super_enum_type {
                    fn from(value: $name) -> Self {
                        Self::$super_enum_case(value.into())
                    }
                }
            )?

            impl TryFrom<Type> for $name {
                type Error = Error;

                fn try_from(value: Type) -> Result<Self> {
                    if let Type::$name(value) = value {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonType)
                }
            }

            impl TryFrom<Micheline> for $name {
                type Error = Error;

                fn try_from(value: Micheline) -> Result<Self> {
                    let primitive_application: PrimitiveApplication = value.try_into()?;

                    primitive_application.try_into()
                }
            }

            impl TryFrom<PrimitiveApplication> for $name {
                type Error = Error;

                #[allow(unused)]
                fn try_from(value: PrimitiveApplication) -> Result<Self> {
                    if value.prim() != Self::prim_value().name() {
                        return Err(Error::InvalidPrimitiveApplication);
                    }
                    let metadata: TypeFieldMetadata = (&value).try_into()?;
                    let mut args = value.into_args().unwrap_or(vec![]);
                    Ok(Self {
                        $(
                            $field_name: if !args.is_empty() { args.remove(0).try_into()? } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        $(
                            $boxed_field_name: if !args.is_empty() { Box::new(args.remove(0).try_into()?) } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        $(
                            $vec_field_name: if !args.is_empty() { args.into_iter().map(|value| value.try_into()).collect::<Result<Vec<_>>>()? } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        metadata,
                    })
                }
            }

            impl From<$name> for Micheline {
                #[allow(unused)]
                fn from(value: $name) -> Self {
                    let mut args: Vec<Micheline> = vec![];
                    let annots: Vec<alloc::string::String> = value.annotations().into_iter().map(|annot| annot.value().into()).collect();
                    $(
                        args.push(value.$field_name.into());
                    )*
                    $(
                        args.push((*value.$boxed_field_name).into());
                    )*
                    $(
                        let mut values = value.$vec_field_name.into_iter().map(|value| value.into()).collect::<Vec<Micheline>>();
                        args.append(&mut values);
                    )*
                    let primitive_application = PrimitiveApplication::new($name::prim_value().name().into(), Some(args), Some(annots));

                    primitive_application.into()
                }
            }

            impl From<&$name> for Micheline {
                #[allow(unused)]
                fn from(value: &$name) -> Self {
                    let mut args: Vec<Micheline> = vec![];
                    let annots: Vec<alloc::string::String> = value.annotations().into_iter().map(|annot| annot.value().into()).collect();
                    $(
                        args.push((&value.$field_name).into());
                    )*
                    $(
                        args.push((&*value.$boxed_field_name).into());
                    )*
                    $(
                        let mut values = value.$vec_field_name.iter().map(|value| value.into()).collect::<Vec<Micheline>>();
                        args.append(&mut values);
                    )*
                    let primitive_application = PrimitiveApplication::new($name::prim_value().name().into(), Some(args), Some(annots));

                    primitive_application.into()
                }
            }

            pub fn $code<Output>($($field_name: $field_type,)* $($boxed_field_name: $boxed_field_type,)* $($vec_field_name: Vec<$vec_field_type>,)*) -> Output where Output: From<$name> {
                $name::new(
                    $($field_name,)*
                    $($boxed_field_name,)*
                    $($vec_field_name,)*
                    None,
                ).into()
            }
        }
    };
}

pub(crate) use {make_type, make_types};
