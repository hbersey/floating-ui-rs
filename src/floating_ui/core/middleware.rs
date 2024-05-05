use std::sync::Arc;

use js_sys::{Function, Object, Promise};
use wasm_bindgen::prelude::*;

use crate::{
    utils_::object_utils::{self, get_option_js_value},
    Error, Result,
};

use super::{MiddlewareReturn, MiddlewareState, Promisable};

#[derive(Clone)]
pub struct Middleware {
    pub name: String,
    pub options: Option<JsValue>,
    pub r#fn: Arc<dyn Fn(MiddlewareState) -> Promisable<MiddlewareReturn>>,
}

impl TryFrom<JsValue> for Middleware {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Middleware> {
        let obj = Object::from(value.clone());

        let name = object_utils::get_string(&obj, "name")?;
        let options = get_option_js_value(&obj, "options")?;

        let fn_val = object_utils::get_js_value(&obj, "fn")?;
        let js_fn = Function::from(fn_val);

        let r#fn = move |state: MiddlewareState| {
            if let Ok(state) = state.try_into() {
                if let Ok(js_promise) = js_fn.call1(&value, &state) {
                    let promise: Promise = js_promise.into();
                    return Promisable::from_promise(promise);
                }
                panic!("Failed to call Middleware function");
            }

            panic!("Failed to convert MiddlewareState");
        };

        Ok(Self {
            name,
            options,
            r#fn: Arc::new(r#fn),
        })
    }
}

impl TryInto<JsValue> for Middleware {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = Object::new();

        object_utils::set_string(&obj, "name", &self.name)?;
        object_utils::set_option_js_value(&obj, "options", self.options)?;

        let r#fn = Closure::wrap(Box::new(move |state: JsValue| {
            if let Ok(state) = MiddlewareState::try_from(state) {
                let res: Promisable<MiddlewareReturn> = (self.r#fn)(state);

                if let Ok(res) = res.try_into_js_value() {
                    return res;
                }
                panic!("Failed to convert MiddlewareReturn");
            }

            panic!("Failed to convert MiddlewareState");
        }) as Box<dyn Fn(JsValue) -> JsValue>);

        object_utils::set_js_value(&obj, "fn", r#fn.as_ref().unchecked_ref())?;

        r#fn.forget();

        Ok(obj.into())
    }
}
