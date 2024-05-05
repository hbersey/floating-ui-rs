mod string_union;
pub(crate) use string_union::string_union;
pub use string_union::StringUnion;

mod enum_union;
pub(crate) use enum_union::enum_union;

mod index;
pub(crate) use index::index;

mod key_in;
pub(crate) use key_in::key_in;

mod extends;
pub(crate) use extends::extends;
use wasm_bindgen::JsValue;

pub mod object_utils;

mod maybe_from;
pub use maybe_from::MaybeFrom;

pub trait TryMaybeFrom<T> {
    type Error;

    fn try_maybe_from(value: T) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized;
}

impl<T, E> TryMaybeFrom<JsValue> for T
where
    T: TryFrom<JsValue, Error = E>,
{
    type Error = E;

    fn try_maybe_from(value: JsValue) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized,
    {
        if value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(Self::try_from(value)?))
    }
}
