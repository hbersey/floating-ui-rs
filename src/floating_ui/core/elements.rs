use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use crate::{utils_::object_utils, Error, Result};

#[derive(Clone)]
pub struct Elements {
    pub reference: HtmlElement,
    pub floating: HtmlElement,
}

impl TryFrom<JsValue> for Elements {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = js_sys::Object::from(value);

        let reference = object_utils::get_js_value(&obj, "reference")?.into();
        let floating = object_utils::get_js_value(&obj, "floating")?.into();

        Ok(Self {
            reference,
            floating,
        })
    }
}

impl TryInto<JsValue> for Elements {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_js_value(&obj, "reference", &self.reference.into())?;
        object_utils::set_js_value(&obj, "floating", &self.floating.into())?;

        Ok(obj.into())
    }
}
