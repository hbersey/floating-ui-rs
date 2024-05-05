use wasm_bindgen::JsValue;

pub trait MaybeFrom<T> {
    fn maybe_from(value: T) -> Option<Self>
    where
        Self: Sized;
}

impl<T> MaybeFrom<JsValue> for T
where
    T: From<JsValue>,
{
    fn maybe_from(value: JsValue) -> Option<Self> {
        if value.is_undefined() {
            None
        } else {
            Some(Self::from(value))
        }
    }
}

