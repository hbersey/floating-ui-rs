use std::result::Result as StdResult;
use thiserror::Error as ThisError;
use wasm_bindgen::JsValue;

#[derive(ThisError, Debug, Clone)]
pub enum Error {
    #[error("Failed to parse `{0}` for {1}")]
    InvalidValue(String, &'static str),

    #[error("Expected `{0}` to be a `{0}::{1}`")]
    InvalidEnumConversion(&'static str, &'static str),

    #[error("Expected `{0}` to be a `{1}`")]
    InvalidJsValue(String, &'static str),

    #[error("Failed to get key `{0}`. {1} is not an object.")]
    ReflectGet(String, String),

    #[error("Failed to set key `{0}`. {1}")]
    ReflectSet(String, String),

    #[error("An unknown error occurred")]
    Unknown,
}

impl Error {
    pub fn map_reflect_get(key: String) -> impl Fn(JsValue) -> Error {
        move |js_error| {
            Error::ReflectSet(
                key.clone(),
                js_error.as_string().expect("TypeError expected"),
            )
        }
    }

    pub fn map_reflect_set(key: String) -> impl Fn(JsValue) -> Error {
        move |js_error| {
            Error::ReflectSet(
                key.clone(),
                format!(
                    "`{}` is not an object.",
                    js_error.as_string().expect("TypeError expected")
                ),
            )
        }
    }
}

pub type Result<T> = StdResult<T, Error>;
