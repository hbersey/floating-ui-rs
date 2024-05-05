use js_sys::{Array, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{
    core::{Middleware, MiddlewareData},
    utils::{Axis, Coords, Placement, Strategy},
    utils_::{extends, index, object_utils},
    Error, Result,
};

#[wasm_bindgen(js_namespace = ["window", "FloatingUIDOM"])]
extern "C" {
    #[wasm_bindgen]
    fn computePosition(
        referenceEl: &HtmlElement,
        floatingEl: &HtmlElement,
        options: JsValue,
    ) -> Promise;
}

#[derive(Clone)]
pub struct ComputePositionOptions {
    placement: Option<Placement>,
    strategy: Option<Strategy>,
    middleware: Option<Vec<Option<Middleware>>>,
}

impl ComputePositionOptions {
    pub fn new() -> Self {
        Self {
            placement: None,
            strategy: None,
            middleware: None,
        }
    }

    pub fn placement(&mut self, placement: Placement) -> &mut Self {
        self.placement = Some(placement);
        self
    }

    pub fn strategy(&mut self, strategy: Strategy) -> &mut Self {
        self.strategy = Some(strategy);
        self
    }

    pub fn middleware(&mut self, middleware: Middleware) -> &mut Self {
        if let Some(middleware_vec) = &mut self.middleware {
            middleware_vec.push(Some(middleware));
        } else {
            self.middleware = Some(vec![Some(middleware)]);
        }
        self
    }

    pub fn maybe_middleware(&mut self, middleware: Option<Middleware>) -> &mut Self {
        if let Some(middleware) = middleware {
            self.middleware(middleware);
        } else {
            if let Some(middleware_vec) = &mut self.middleware {
                middleware_vec.push(None);
            } else {
                self.middleware = Some(vec![None]);
            }
        }
        self
    }
}

impl TryInto<JsValue> for ComputePositionOptions {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_option_js_value(&obj, "placement", self.placement.map(Placement::into))?;
        object_utils::set_option_js_value(&obj, "strategy", self.strategy.map(Strategy::into))?;

        if let Some(middleware) = self.middleware {
            let js_array = Array::new_with_length(middleware.len() as u32);
            for m in middleware {
                let js_m: JsValue = match m {
                    Some(m) => m.try_into()?,
                    None => JsValue::NULL,
                };

                js_array.push(&js_m);
            }

            let js_val: JsValue = js_array.into();
            object_utils::set_js_value(&obj, "middleware", &js_val)?;
        }

        Ok(obj.into())
    }
}

pub struct ComputePositionReturn {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
}

extends!(ComputePositionReturn => Coords, x, y);
index!(ComputePositionReturn, Axis {X, Y} => f64);

pub struct ActualComputePositionReturn(Promise);

impl ActualComputePositionReturn {
    pub fn then<F>(self, f: F)
    where
        F: FnOnce(ComputePositionReturn) + 'static,
    {
        let cb = Closure::once(|js_val: JsValue| {
            let obj = Object::from(js_val);

            let x = Reflect::get(&obj, &"x".into())
                .expect("Failed to get x")
                .as_f64()
                .expect("Failed to convert x to f64");

            let y = Reflect::get(&obj, &"y".into())
                .expect("Failed to get y")
                .as_f64()
                .expect("Failed to convert y to f64");

            let placement: Placement = Reflect::get(&obj, &"placement".into())
                .expect("Failed to get placement")
                .try_into()
                .expect(
                    format!(
                        "Failed to convert JsValue to Placement, {:?}",
                        Reflect::get(&obj, &"placement".into()).unwrap(),
                    )
                    .as_str(),
                );

            let strategy: Strategy = Reflect::get(&obj, &"strategy".into())
                .expect("Failed to get strategy")
                .try_into()
                .expect("Failed to convert JsValue to Strategy");

            let middleware_data: MiddlewareData = Reflect::get(&obj, &"middlewareData".into())
                .expect("Failed to get middlewareData")
                .try_into()
                .expect("Failed to convert JsValue to MiddlewareData");

            f(ComputePositionReturn {
                x,
                y,
                placement,
                strategy,
                middleware_data,
            });
        });
        let _ = self.0.then(&cb);
        cb.forget();
    }
}

pub fn compute_position(
    reference_el: &HtmlElement,
    floating_el: &HtmlElement,
    options: ComputePositionOptions,
) -> Result<ActualComputePositionReturn> {
    let promise: Promise = computePosition(reference_el, floating_el, options.try_into()?);
    Ok(ActualComputePositionReturn(promise))
}
