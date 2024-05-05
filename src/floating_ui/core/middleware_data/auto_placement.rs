use crate::{utils::Placement, utils_::object_utils, Error, Result};

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;

#[derive(Default, Clone)]
pub struct AutoPlacement {
    pub index: Option<f64>,
    pub overflows: Vec<OverflowItem>,
}

impl TryInto<JsValue> for AutoPlacement {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = Object::new();

        object_utils::set_option_f64(&obj, "index", self.index)?;

        let overflows = js_sys::Array::new();
        for overflow in self.overflows {
            let js: JsValue = overflow.try_into()?;
            overflows.push(&js);
        }
        object_utils::set_js_value(&obj, "overflows", &overflows.into())?;

        Ok(obj.into())
    }
}

impl TryFrom<JsValue> for AutoPlacement {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = Object::from(value);

        let index = object_utils::get_option_f64(&obj, "index")?;

        let js_overflows = Array::from(&object_utils::get_js_value(&obj, "overflows")?);
        let mut overflows = Vec::new();
        for val in js_overflows {
            let overflow = OverflowItem::try_from(val)?;
            overflows.push(overflow);
        }

        Ok(Self { index, overflows })
    }
}

#[derive(Clone)]
pub struct OverflowItem {
    pub placement: Placement,
    pub overflows: Vec<f64>,
}

impl TryInto<JsValue> for OverflowItem {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = Object::new();

        object_utils::set_js_value(&obj, "placement", &self.placement.into())?;

        let overflows = js_sys::Array::new();
        for overflow in self.overflows {
            overflows.push(&JsValue::from_f64(overflow));
        }
        object_utils::set_js_value(&obj, "overflows", &overflows.into())?;

        Ok(obj.into())
    }
}

impl TryFrom<JsValue> for OverflowItem {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = Object::from(value);

        let placement = Reflect::get(&obj, &"placement".into()).unwrap();
        let overflows = Reflect::get(&obj, &"overflows".into()).unwrap();

        let placement: Placement = placement.try_into()?;
        let overflows = js_sys::Array::from(&overflows)
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            placement,
            overflows,
        })
    }
}
