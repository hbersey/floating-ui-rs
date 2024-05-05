use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{
    error::{Error, Result},
    utils_::object_utils,
};

#[wasm_bindgen(js_namespace=["window", "FloatingUIDOM"])]
extern "C" {
    #[wasm_bindgen]
    fn autoUpdate(
        reference: HtmlElement,
        floating: HtmlElement,
        update: &Closure<dyn Fn()>,
        options: JsValue,
    );
}

pub fn auto_update<F>(
    reference: HtmlElement,
    floating: HtmlElement,
    update: F,
    options: AutoUpdateOptions,
) -> Result<()>
where
    F: Fn() + 'static,
{
    let update: Closure<dyn Fn()> = Closure::new(update);
    let options: JsValue = options.try_into()?;

    // TODO cleanup type
    let cleanup = autoUpdate(reference, floating, &update, options);

    update.forget();

    Ok(cleanup)
}
pub struct AutoUpdateOptions {
    ancestor_scroll: Option<bool>,
    ancestor_resize: Option<bool>,
    element_resize: Option<bool>,
    layout_shift: Option<bool>,
    animation_frame: Option<bool>,
}

impl AutoUpdateOptions {
    pub fn new() -> Self {
        Self {
            ancestor_scroll: None,
            ancestor_resize: None,
            element_resize: None,
            layout_shift: None,
            animation_frame: None,
        }
    }

    pub fn ancestor_scroll(&mut self) -> &mut Self {
        self.ancestor_scroll = Some(true);
        self
    }

    pub fn ancestor_resize(&mut self) -> &mut Self {
        self.ancestor_resize = Some(true);
        self
    }

    pub fn element_resize(&mut self) -> &mut Self {
        self.element_resize = Some(true);
        self
    }

    pub fn layout_shift(&mut self) -> &mut Self {
        self.layout_shift = Some(true);
        self
    }

    pub fn animation_frame(&mut self) -> &mut Self {
        self.animation_frame = Some(true);
        self
    }
}

impl TryInto<JsValue> for AutoUpdateOptions {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = Object::new();

        object_utils::set_option_bool(&obj, "ancestorScroll", self.ancestor_scroll)?;
        object_utils::set_option_bool(&obj, "ancestorResize", self.ancestor_resize)?;
        object_utils::set_option_bool(&obj, "elementResize", self.element_resize)?;
        object_utils::set_option_bool(&obj, "layoutShift", self.layout_shift)?;
        object_utils::set_option_bool(&obj, "animationFrame", self.animation_frame)?;

        Ok(obj.into())
    }
}
