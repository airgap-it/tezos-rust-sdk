macro_rules! make_encoded_structs {
    (
        $(
            {
                $(
                    use {
                        $(
                            $type_path:path,
                        )+
                    }
                )?
                struct $mod_name:ident::$name:ident;
                coder: $coder:ty;
                meta {
                    base58_prefix: $b58_prefix:literal,
                    base58_length: $b58_length:literal,
                    bytes_prefix: [$($b_prefix:literal, )+],
                    bytes_length: $b_length:literal,
                }
                $(
                    extra_try_from {
                        $(
                            $try_from_type:ty,
                        )+
                    }
                )?
                $(
                    test {
                        string_value: $test_value:literal,
                        bytes_value: [$($test_bytes:literal, )+],
                    }
                )?
            },
        )+
    ) => {
        pub use self::{
            $(
                $mod_name::$name,
            )+
        };

        const META_ENCODED_VALUES: &[&'static $crate::types::encoded::MetaEncoded] = &[
            $(
                &$mod_name::META,
            )+
        ];

        $(
            make_encoded_struct!(
                $(
                    use {
                        $(
                            $type_path,
                        )+
                    }
                )?
                struct $mod_name::$name;
                coder: $coder;
                meta {
                    base58_prefix: $b58_prefix,
                    base58_length: $b58_length,
                    bytes_prefix: [$($b_prefix, )+],
                    bytes_length: $b_length,
                }
                $(
                    extra_try_from {
                        $(
                            $try_from_type,
                        )+
                    }
                )?
                $(
                    test {
                        string_value: $test_value,
                        bytes_value: [$($test_bytes, )+],
                    }
                )?
            );
        )+
    };
}

macro_rules! make_encoded_struct {
    (
        $(
            use {
                $(
                    $type_path:path,
                )+
            }
        )?
        struct $mod_name:ident::$name:ident;
        coder: $coder:ty;
        meta {
            base58_prefix: $b58_prefix:literal,
            base58_length: $b58_length:literal,
            bytes_prefix: [$($b_prefix:literal, )+],
            bytes_length: $b_length:literal,
        }
        $(
            extra_try_from {
                $(
                    $try_from_type:ty,
                )+
            }
        )?
        $(
            test {
                string_value: $test_value:literal,
                bytes_value: [$($test_bytes:literal, )+],
            }
        )?
    ) => {
        mod $mod_name {
            $(
                use $crate::{
                    $($type_path, )+
                };
            )?
            use $crate::{
                types::encoded::{Encoded, MetaEncoded, TraitMetaEncoded},
                Error, Result,
                internal::{consumable_list::ConsumableList, coder::ConsumingDecoder},
            };
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};
            use alloc::string::{ToString, String};
            use alloc::vec::Vec;

            /// Structure representing a base58 encoded Tezos value
            #[derive(Debug, Clone, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde( try_from = "String"))]
            pub struct $name(String);

            impl $name {
                pub fn is_valid_base58(value: &str) -> bool {
                    META.is_valid_base58(value)
                }

                pub fn is_valid_bytes(value: &[u8]) -> bool {
                    META.is_valid_bytes(value)
                }

                pub fn is_valid_prefixed_bytes(value: &[u8]) -> bool {
                    META.is_valid_prefixed_bytes(value)
                }

                pub fn is_valid_consumable_bytes(value: &[u8]) -> bool {
                    META.is_valid_consumable_bytes(value)
                }

                pub fn is_valid_prefixed_consumable_bytes(value: &[u8]) -> bool {
                    META.is_valid_prefixed_consumable_bytes(value)
                }
            }

            impl Encoded for $name {
                type Coder = $coder;

                fn value(&self) -> &str {
                    &self.0
                }

                fn meta(&self) -> &'static MetaEncoded {
                    &META
                }

                fn new(value: String) -> Result<Self> {
                    if META.is_valid_base58(&value) {
                        return Ok($name(value));
                    }
                    return Err(Error::InvalidBase58EncodedData { description: value });
                }

                fn from_bytes(bytes: &[u8]) -> Result<Self> {
                    Self::Coder::decode_with_meta(bytes, &META)
                }

                fn from_consumable_bytes<CL: ConsumableList<u8>>(bytes: &mut CL) -> Result<Self>
                where
                    Self::Coder: ConsumingDecoder<Self, u8, Error>,
                {
                    Self::Coder::decode_consuming_with_meta(bytes, &META)
                }
            }

            impl TraitMetaEncoded for $name {
                fn meta_value() -> &'static MetaEncoded {
                    &META
                }
            }

            pub const META: MetaEncoded = MetaEncoded::new($b58_prefix, $b58_length, &[$($b_prefix, )+], $b_length);

            impl From<$name> for String {
                fn from(value: $name) -> Self {
                    value.0
                }
            }

            impl TryFrom<&Vec<u8>> for $name {
                type Error = Error;

                fn try_from(value: &Vec<u8>) -> Result<Self> {
                    Self::from_bytes(value)
                }
            }

            impl TryFrom<[u8; META.bytes_length]> for $name {
                type Error = Error;

                fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
                    <Self as Encoded>::Coder::decode_with_meta(&value, &META)
                }
            }

            impl TryFrom<String> for $name {
                type Error = Error;

                fn try_from(value: String) -> Result<Self> {
                    Self::new(value)
                }
            }

            impl TryFrom<&str> for $name {
                type Error = Error;

                fn try_from(value: &str) -> Result<Self> {
                    Self::new(value.to_string())
                }
            }

            impl TryFrom<&$name> for Vec<u8> {
                type Error = Error;

                fn try_from(value: &$name) -> Result<Self> {
                    value.to_bytes()
                }
            }

            $(
                $(
                    impl TryFrom<$try_from_type> for $name {
                        type Error = Error;

                        fn try_from(value: $try_from_type) -> Result<Self> {
                            let bytes = value.to_bytes()?;
                            (&bytes).try_into()
                        }
                    }
                )+
            )?

            $(
                #[cfg(test)]
                mod test {
                    use super::*;

                    #[test]
                    fn test_convert_from_string() -> Result<()> {
                        let value: $name = $test_value.try_into()?;
                        assert_eq!(value.value(), $test_value);
                        assert_eq!(value.to_bytes()?, &[$($test_bytes, )+]);

                        Ok(())
                    }

                    #[test]
                    fn test_convert_from_bytes() -> Result<()> {
                        let value: $name = [$($test_bytes, )+].try_into()?;
                        assert_eq!(value.value(), $test_value);

                        Ok(())
                    }
                }
            )?
        }
    };
}

pub(crate) use {make_encoded_struct, make_encoded_structs};
