use js_sys::Promise;
use wasm_bindgen::JsValue;

use crate::{Error, Result};

#[derive(Clone)]
pub enum Promisable<T> {
    T(T),
    Promise(Promise),
}

impl<T> Promisable<T> {
    pub fn from_promise(promise: Promise) -> Self {
        Self::Promise(promise)
    }
}

impl<T> From<T> for Promisable<T> {
    fn from(t: T) -> Self {
        Self::T(t)
    }
}

impl<T> Into<JsValue> for Promisable<T>
where
    T: Into<JsValue>,
{
    fn into(self) -> JsValue {
        match self {
            Promisable::T(t) => t.into(),
            Promisable::Promise(promise) => promise.into(),
        }
    }
}

impl<T> Promisable<T> {
    pub fn try_into_js_value(self) -> Result<JsValue>
    where
        T: TryInto<JsValue, Error = Error>,
    {
        match self {
            Promisable::T(t) => t.try_into(),
            Promisable::Promise(promise) => Ok(promise.into()),
        }
    }
}
