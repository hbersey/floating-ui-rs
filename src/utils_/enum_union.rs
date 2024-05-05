macro_rules! enum_union {
    ($e_name:ident, $($variant:ty),*) => {
        paste::paste! {
            #[derive(Clone)]
            pub enum $e_name {
                $(
                    [<$variant:camel>]($variant),
                )*
            }

            $(
                impl From<[<$variant:camel>]> for $e_name {
                    fn from(v: [<$variant:camel>]) -> Self {
                        Self::$variant(v)
                    }
                }


                impl TryInto<[<$variant:camel>]> for $e_name {
                    type Error = crate::Error;

                    fn try_into(self) -> crate::Result<$variant> {
                        match self {
                            Self::$variant(v) => Ok(v),
                            _ => Err(crate::Error::InvalidEnumConversion(stringify!($e_name), stringify!($variant))),
                        }
                    }
                }

            )*
        }
    };
}

pub(crate) use enum_union;
