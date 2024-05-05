use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::{
    utils::{ElementRects, Placement},
    utils_::object_utils,
    Error, Result,
};

#[derive(Clone)]
pub struct MiddlewareReturn {
    pub data: Option<Object>,
    pub reset: Option<Reset>,
}

impl From<JsValue> for MiddlewareReturn {
    fn from(value: JsValue) -> Self {
        let data = js_sys::Reflect::get(&value, &JsValue::from_str("data"));

        let data = if let Ok(val) = data {
            if val.is_undefined() {
                None
            } else {
                Some(Object::from(val))
            }
        } else {
            None
        };

        let reset = js_sys::Reflect::get(&value, &JsValue::from_str("reset"));
        let reset = if let Ok(val) = reset {
            if val.is_undefined() {
                None
            } else {
                Some(Reset::Object {
                    placement: None,
                    rects: None,
                })
            }
        } else {
            None
        };

        Self { data, reset }
    }
}

impl TryInto<JsValue> for MiddlewareReturn {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        if let Some(data) = self.data {
            object_utils::set_js_value(&obj, "data", &data.into())?;
        }

        if let Some(reset) = self.reset {
            let js_reset = match reset {
                Reset::Bool(val) => JsValue::from_bool(val),
                Reset::Object { placement, rects } => {
                    let obj = js_sys::Object::new();

                    if let Some(placement) = placement {
                        object_utils::set_js_value(&obj, "placement", &placement.into())?;
                    }

                    object_utils::try_set_option_js_value(&obj, "rects", rects)?;

                    obj.into()
                }
            };
            object_utils::set_js_value(&obj, "reset", &js_reset)?;
        }

        Ok(obj.into())
    }
}

#[derive(Clone)]
pub enum Reset {
    Bool(bool),
    Object {
        placement: Option<Placement>,
        rects: Option<ElementRects>,
    },
}

pub enum ResetRects {
    Bool(bool),
    ElementRects(ElementRects),
}
