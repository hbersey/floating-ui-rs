use wasm_bindgen::JsValue;

use crate::{
    utils::{Axis, Coords, ElementRects, Placement, Strategy},
    utils_::{extends, index, object_utils},
    Error, Result,
};

use super::{Elements, MiddlewareData};

#[derive(Clone)]
pub struct MiddlewareState {
    pub x: f64,
    pub y: f64,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
    pub elements: Elements,
    pub rects: ElementRects,
    // TODO platform
    pub platform: JsValue,
}

extends!(MiddlewareState => Coords, x, y);
index!(MiddlewareState, Axis {X, Y} => f64);

impl TryFrom<JsValue> for MiddlewareState {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = js_sys::Object::from(value);

        let x = object_utils::get_f64(&obj, "x")?;
        let y = object_utils::get_f64(&obj, "y")?;
        let initial_placement = object_utils::get_js_value(&obj, "initialPlacement")?.try_into()?;
        let placement = object_utils::get_js_value(&obj, "placement")?.try_into()?;
        let strategy = object_utils::get_js_value(&obj, "strategy")?.try_into()?;
        let middleware_data = object_utils::get_js_value(&obj, "middlewareData")?.try_into()?;
        let elements = object_utils::get_js_value(&obj, "elements")?.try_into()?;
        let rects = object_utils::get_js_value(&obj, "rects")?.try_into()?;
        let platform = object_utils::get_js_value(&obj, "platform")?;

        Ok(Self {
            x,
            y,
            initial_placement,
            placement,
            strategy,
            middleware_data,
            elements,
            rects,
            platform,
        })
    }
}

impl TryInto<JsValue> for MiddlewareState {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_f64(&obj, "x", self.x)?;
        object_utils::set_f64(&obj, "y", self.y)?;
        object_utils::set_js_value(&obj, "y", &self.y.into())?;
        object_utils::set_js_value(&obj, "initialPlacement", &self.initial_placement.into())?;
        object_utils::set_js_value(&obj, "placement", &self.placement.into())?;
        object_utils::set_js_value(&obj, "strategy", &self.strategy.into())?;
        object_utils::try_set_js_value(&obj, "middlewareData", self.middleware_data)?;
        object_utils::try_set_js_value(&obj, "elements", self.elements)?;
        object_utils::try_set_js_value(&obj, "rects", self.rects)?;
        object_utils::set_js_value(&obj, "platform", &self.platform)?;

        Ok(obj.into())
    }
}
