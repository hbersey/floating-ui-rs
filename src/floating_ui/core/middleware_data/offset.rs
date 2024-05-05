use wasm_bindgen::JsValue;

use crate::{
    utils::{Axis, Coords, Placement},
    utils_::{extends, index, object_utils},
    Error, Result,
};

#[derive(Clone)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
}

extends!(Offset => Coords, x, y);
index!(Offset, Axis { X, Y } => f64);

impl TryInto<JsValue> for Offset {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_f64(&obj, "x", self.x)?;
        object_utils::set_f64(&obj, "y", self.y)?;
        object_utils::set_js_value(&obj, "placement", &self.placement.into())?;

        Ok(obj.into())
    }
}

impl TryFrom<JsValue> for Offset {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Offset> {
        let obj = js_sys::Object::from(value);

        let x = object_utils::get_f64(&obj, "x")?;
        let y = object_utils::get_f64(&obj, "y")?;
        let placement = object_utils::get_js_value(&obj, "placement")?.try_into()?;

        Ok(Self { x, y, placement })
    }
}
