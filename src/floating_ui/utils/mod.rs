use crate::{
    utils_::{key_in, object_utils, string_union, StringUnion},
    Error, Result,
};

string_union!(Alignment {
    "start" => Start,
    "end" => End
});

string_union!(Side {
    "top" => Top,
    "bottom" => Bottom,
    "left" => Left,
    "right" => Right
});

mod aligned_placement;
pub use aligned_placement::AlignmentPlacement;

mod placement;
pub use placement::Placement;

string_union!(Strategy {
    "absolute" => Absolute,
    "fixed" => Fixed
});

string_union!(Axis {
    "x" => X,
    "y" => Y
});

key_in!(Coords, Axis { X, Y } => f64);

string_union!(Length {
    "width" => Width,
    "height" => Height
});

key_in!(Dimensions, Length { Width, Height } => f64);

mod side_object;
pub use side_object::{PartialSideObject, SideObject};

mod rect;
pub use rect::Rect;

mod padding;
pub use padding::Padding;

mod client_rect_object;
pub use client_rect_object::ClientRectObject;
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

impl TryFrom<JsValue> for ElementRects {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let obj = js_sys::Object::from(value);

        let reference = object_utils::get_js_value(&obj, "reference")?.try_into()?;
        let floating = object_utils::get_js_value(&obj, "floating")?.try_into()?;

        Ok(Self {
            reference,
            floating,
        })
    }
}

impl TryInto<JsValue> for ElementRects {
    type Error = Error;

    fn try_into(self) -> Result<JsValue> {
        let obj = js_sys::Object::new();

        object_utils::set_js_value(&obj, "reference", &self.reference.try_into()?)?;
        object_utils::set_js_value(&obj, "floating", &self.floating.try_into()?)?;

        Ok(obj.into())
    }
}
