macro_rules! make_primitive_enum {
    (
        $($name:ident, $code:ident, $tag:literal)+
    ) => {
        /// Enum encapsulating all the various Tezos primitive types and values.
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum Primitive {
            $($name = $tag,)*
        }

        impl Primitive {
            pub fn to_u8(&self) -> u8 {
                (*self).into()
            }

            pub fn to_str(&self) -> &'static str {
                (*self).into()
            }
        }

        impl alloc::str::FromStr for Primitive {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $(stringify!($code) => Ok(Self::$name),)*
                    _ => Err(Error::InvalidStringValue),
                }
            }
        }

        impl TryFrom<u8> for Primitive {
            type Error = Error;

            fn try_from(value: u8) -> Result<Self> {
                match value {
                    $($tag => Ok(Self::$name),)*
                    _ => Err(Error::InvalidBytes)
                }
            }
        }

        impl From<Primitive> for &'static str {
            fn from(value: Primitive) -> Self {
                match value {
                    $(Primitive::$name => stringify!($code),)*
                }
            }
        }

        impl From<Primitive> for alloc::string::String {
            fn from(value: Primitive) -> Self {
                match value {
                    $(Primitive::$name => stringify!($code).into(),)*
                }
            }
        }

        impl From<Primitive> for u8 {
            fn from(value: Primitive) -> Self {
                value as u8
            }
        }
    };
}

pub(crate) use make_primitive_enum;
