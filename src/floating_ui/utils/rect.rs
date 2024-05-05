use wasm_bindgen::JsValue;

use super::{Axis, Coords, Dimensions, Length};
use crate::{
    utils_::{extends, index, object_utils},
    Error, Result,
};

#[derive(Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

extends!(Rect => Coords, x, y);
index!(Rect, Axis { X, Y } => f64);

extends!(Rect => Dimensions, width, height);
index!(Rect, Length { Width, Height } => f64);

impl TryFrom<JsValue> for Rect {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = js_sys::Object::from(value);

        let x = object_utils::get_f64(&obj, "x")?;
        let y = object_utils::get_f64(&obj, "y")?;
        let width = object_utils::get_f64(&obj, "width")?;
        let height = object_utils::get_f64(&obj, "height")?;

        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }
}

impl TryInto<JsValue> for Rect {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_f64(&obj, "x", self.x)?;
        object_utils::set_f64(&obj, "y", self.y)?;
        object_utils::set_f64(&obj, "width", self.width)?;
        object_utils::set_f64(&obj, "height", self.height)?;

        Ok(obj.into())
    }
}
