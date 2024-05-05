use wasm_bindgen::JsValue;

use crate::{
    error::{Error, Result},
    utils::SideObject,
    utils_::object_utils,
};

#[derive(Default, Clone)]
pub struct Hide {
    pub reference_hidden: Option<bool>,
    pub escaped: Option<bool>,
    pub reference_hidden_offsets: Option<SideObject>,
    pub escaped_offsets: Option<SideObject>,
}

impl TryInto<JsValue> for Hide {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_option_bool(&obj, "referenceHidden", self.reference_hidden)?;
        object_utils::set_option_bool(&obj, "escaped", self.escaped)?;
        object_utils::try_set_option_js_value(
            &obj,
            "referenceHiddenOffsets",
            self.reference_hidden_offsets,
        )?;
        object_utils::try_set_option_js_value(&obj, "escapedOffsets", self.escaped_offsets)?;

        Ok(obj.into())
    }
}

impl TryFrom<JsValue> for Hide {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = js_sys::Object::from(value);

        let reference_hidden = object_utils::get_option_bool(&obj, "referenceHidden")?;
        let escaped = object_utils::get_option_bool(&obj, "escaped")?;
        let reference_hidden_offsets =
            object_utils::try_get_option::<SideObject>(&obj, "referenceHiddenOffsets")?;
        let escaped_offsets = object_utils::try_get_option::<SideObject>(&obj, "escapedOffsets")?;

        Ok(Self {
            reference_hidden,
            escaped,
            reference_hidden_offsets,
            escaped_offsets,
        })
    }
}
