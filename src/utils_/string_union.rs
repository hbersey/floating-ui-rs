use crate::error::Result;

pub trait StringUnion: Sized {
    fn as_str(&self) -> &str;
    fn try_from_str(value: &str) -> Result<Self>;
}

macro_rules! string_union {
    ($name:ident {
        $($str:literal => $variant:ident),*
    }) => {
        #[derive(Clone)]
        pub enum $name {
            $($variant),*
        }

        impl crate::utils_::StringUnion for $name {
            fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $str,)*
                }
            }

            fn try_from_str(value: &str) -> crate::Result<Self> {
                match value {
                    $($str => Ok(Self::$variant),)*
                    _ => Err(crate::Error::InvalidValue(value.to_string(), stringify!($name))),
                }
            }
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                self.as_str().to_string()
            }
        }

        impl Into<wasm_bindgen::JsValue> for $name {
            fn into(self) -> wasm_bindgen::JsValue {
                self.as_str().into()
            }
        }

        impl TryFrom<wasm_bindgen::JsValue> for $name {
            type Error = crate::Error;

            fn try_from(value: wasm_bindgen::JsValue) -> crate::Result<Self> {
                let s = match value.as_string() {
                    Some(s) => s,
                    None => return Err(crate::Error::InvalidJsValue("value".to_string(), "string")),
                };

                Self::try_from_str(&s)
            }
        }
    };
}

pub(crate) use string_union;
