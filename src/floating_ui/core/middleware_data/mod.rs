use js_sys::Object;
use wasm_bindgen::JsValue;

use crate::{
    utils::{Coords, Placement, SideObject},
    utils_::object_utils,
    Error, Result,
};

#[derive(Clone)]
pub struct MiddlewareData {
    pub obj: Object,
    pub arrow: Option<Arrow>,
    pub auto_placement: Option<AutoPlacement>,
    pub flip: Option<Flip>,
    pub hide: Option<Hide>,
    pub offset: Option<Offset>,
    pub shift: Option<Coords>,
}

impl MiddlewareData {
    pub fn new() -> Self {
        Self {
            obj: Object::new(),
            arrow: None,
            auto_placement: None,
            flip: None,
            hide: None,
            offset: None,
            shift: None,
        }
    }

    pub fn get(&self, key: &str) -> Result<JsValue> {
        object_utils::get_js_value(&self.obj, key)
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: Into<JsValue>,
    {
        object_utils::set_js_value(&self.obj, key, &value.into())
    }

    pub fn with<T>(mut self, key: &str, value: T) -> Result<Self>
    where
        T: Into<JsValue>,
    {
        self.set(key, value)?;
        Ok(self)
    }

    pub fn arrow_x(mut self, x: f64) -> Self {
        self.arrow = Some(arrow::Arrow {
            x: Some(x),
            ..self.arrow.unwrap_or_default()
        });
        self
    }

    pub fn arrow_y(mut self, y: f64) -> Self {
        self.arrow = Some(arrow::Arrow {
            y: Some(y),
            ..self.arrow.unwrap_or_default()
        });
        self
    }

    pub fn arrow_center_offset(mut self, center_offset: f64) -> Self {
        self.arrow = Some(arrow::Arrow {
            center_offset: Some(center_offset),
            ..self.arrow.unwrap_or_default()
        });
        self
    }

    pub fn arrow_alignment_offset(mut self, alignment_offset: f64) -> Self {
        self.arrow = Some(arrow::Arrow {
            alignment_offset: Some(alignment_offset),
            ..self.arrow.unwrap_or_default()
        });
        self
    }

    pub fn auto_placement_index(mut self, index: f64) -> Self {
        self.auto_placement = Some(auto_placement::AutoPlacement {
            index: Some(index),
            ..self.auto_placement.unwrap_or_default()
        });
        self
    }

    pub fn auto_placement_overflow(mut self, placement: Placement, overflows: Vec<f64>) -> Self {
        let overflow_item = auto_placement::OverflowItem {
            placement,
            overflows,
        };
        self.auto_placement = Some(auto_placement::AutoPlacement {
            overflows: vec![overflow_item],
            ..self.auto_placement.unwrap_or_default()
        });
        self
    }

    pub fn flip_index(mut self, index: f64) -> Self {
        self.flip = Some(auto_placement::AutoPlacement {
            index: Some(index),
            ..self.flip.unwrap_or_default()
        });
        self
    }

    pub fn flip_overflow(mut self, placement: Placement, overflows: Vec<f64>) -> Self {
        let overflow_item = auto_placement::OverflowItem {
            placement,
            overflows,
        };
        self.flip = Some(auto_placement::AutoPlacement {
            overflows: vec![overflow_item],
            ..self.flip.unwrap_or_default()
        });
        self
    }

    pub fn hide_reference_hidden(mut self) -> Self {
        self.hide = Some(hide::Hide {
            reference_hidden: Some(true),
            ..self.hide.unwrap_or_default()
        });
        self
    }

    pub fn hide_escaped(mut self) -> Self {
        self.hide = Some(hide::Hide {
            escaped: Some(true),
            ..self.hide.unwrap_or_default()
        });
        self
    }

    pub fn hide_reference_hidden_offsets(mut self, offsets: SideObject) -> Self {
        self.hide = Some(hide::Hide {
            reference_hidden_offsets: Some(offsets),
            ..self.hide.unwrap_or_default()
        });
        self
    }

    pub fn hide_escaped_offsets(mut self, offsets: SideObject) -> Self {
        self.hide = Some(hide::Hide {
            escaped_offsets: Some(offsets),
            ..self.hide.unwrap_or_default()
        });
        self
    }

    pub fn offset(mut self, x: f64, y: f64, placement: Placement) -> Self {
        self.offset = Some(offset::Offset { x, y, placement });
        self
    }

    pub fn shift(mut self, x: f64, y: f64) -> Self {
        self.shift = Some(Coords { x, y });
        self
    }
}

impl TryFrom<JsValue> for MiddlewareData {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = Object::from(value);

        let arrow = object_utils::try_get_option::<Arrow>(&obj, "arrow")?;
        let auto_placement = object_utils::try_get_option::<AutoPlacement>(&obj, "autoPlacement")?;
        let flip = object_utils::try_get_option::<Flip>(&obj, "flip")?;
        let hide = object_utils::try_get_option::<Hide>(&obj, "hide")?;
        let offset = object_utils::try_get_option::<Offset>(&obj, "offset")?;
        let shift = object_utils::try_get_option::<Coords>(&obj, "shift")?;

        Ok(Self {
            obj,
            arrow,
            auto_placement,
            flip,
            hide,
            offset,
            shift,
        })
    }
}

impl TryInto<JsValue> for MiddlewareData {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = self.obj.clone();

        object_utils::try_set_option_js_value(&obj, "arrow", self.arrow)?;
        object_utils::try_set_option_js_value(&obj, "autoPlacement", self.auto_placement)?;
        object_utils::try_set_option_js_value(&obj, "flip", self.flip)?;
        object_utils::try_set_option_js_value(&obj, "hide", self.hide)?;
        object_utils::try_set_option_js_value(&obj, "offset", self.offset)?;

        Ok(obj.into())
    }
}

mod arrow;
pub use arrow::Arrow;

mod auto_placement;
pub use auto_placement::{AutoPlacement, OverflowItem};

pub type Flip = AutoPlacement;

mod hide;
pub use hide::Hide;

mod offset;
pub use offset::Offset;
