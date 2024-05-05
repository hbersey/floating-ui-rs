use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::{
    utils::{Axis, PartialCoords},
    utils_::{extends, index, object_utils},
    Error, Result,
};

#[derive(Default, Clone)]
pub struct Arrow {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub center_offset: Option<f64>,
    pub alignment_offset: Option<f64>,
}

extends!(Arrow => PartialCoords, x, y);

index!(Arrow, Axis { X, Y } => Option<f64>);

impl TryInto<JsValue> for Arrow {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = Object::new();

        object_utils::set_option_f64(&obj, "x", self.x)?;
        object_utils::set_option_f64(&obj, "y", self.y)?;
        object_utils::set_option_f64(&obj, "centerOffset", self.center_offset)?;
        object_utils::set_option_f64(&obj, "alignmentOffset", self.alignment_offset)?;

        Ok(obj.into())
    }
}

impl TryFrom<JsValue> for Arrow {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = Object::from(value);

        let x = object_utils::get_option_f64(&obj, "x")?;
        let y = object_utils::get_option_f64(&obj, "y")?;
        let center_offset = object_utils::get_option_f64(&obj, "centerOffset")?;
        let alignment_offset = object_utils::get_option_f64(&obj, "alignmentOffset")?;

        Ok(Self {
            x,
            y,
            center_offset,
            alignment_offset,
        })
    }
}
