use wasm_bindgen::prelude::*;

use crate::{core::Middleware, Result};

#[wasm_bindgen(js_namespace= ["window", "FloatingUIDOM"])]
extern "C" {
    #[wasm_bindgen(js_name = offset)]
    fn js_offset(options: JsValue) -> JsValue;

}

pub fn offset(options: OffsetOptions) -> Result<Middleware> {
    let js = js_offset(options.into());
    Ok(js.try_into()?)
}

pub enum OffsetOptions {
    Number(f64),
    Object {
        main_axis: Option<f64>,
        cross_axis: Option<f64>,
        alignment_axis: Option<f64>,
    },
}

impl Into<JsValue> for OffsetOptions {
    fn into(self) -> JsValue {
        match self {
            OffsetOptions::Number(value) => JsValue::from_f64(value),
            OffsetOptions::Object {
                main_axis,
                cross_axis,
                alignment_axis,
            } => {
                let obj = js_sys::Object::new();
                if let Some(value) = main_axis {
                    js_sys::Reflect::set(
                        &obj,
                        &JsValue::from_str("mainAxis"),
                        &JsValue::from_f64(value),
                    )
                    .unwrap();
                }
                if let Some(value) = cross_axis {
                    js_sys::Reflect::set(
                        &obj,
                        &JsValue::from_str("crossAxis"),
                        &JsValue::from_f64(value),
                    )
                    .unwrap();
                }
                if let Some(value) = alignment_axis {
                    js_sys::Reflect::set(
                        &obj,
                        &JsValue::from_str("alignmentAxis"),
                        &JsValue::from_f64(value),
                    )
                    .unwrap();
                }
                obj.into()
            }
        }
    }
}
