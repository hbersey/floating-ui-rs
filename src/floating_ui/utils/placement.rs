use wasm_bindgen::JsValue;

use super::{Alignment, AlignmentPlacement, Side};
use crate::{
    utils_::{enum_union, StringUnion as _},
    Error, Result,
};

macro_rules! impl_placement {
    () => {
        impl Placement {
            impl_placement!(side: Top);
            impl_placement!(side: Right);
            impl_placement!(side: Bottom);
            impl_placement!(side: Left);
        }
    };
    (side: $side:ident) => {
        paste::paste! {
            pub fn [<$side:snake>]()-> Self {
                Placement::Side(Side::$side)
            }

            pub fn [<$side:snake _start>]() -> Self {
                Placement::AlignmentPlacement(AlignmentPlacement{ side: Side::$side, alignment: Alignment::Start })
            }

            pub fn [<$side:snake _end>]() -> Self {
                Placement::AlignmentPlacement(AlignmentPlacement{ side: Side::$side, alignment: Alignment::End })
            }
        }
    }
}

enum_union!(Placement, Side, AlignmentPlacement);
impl_placement!();

impl TryFrom<JsValue> for Placement {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let s = value
            .as_string()
            .ok_or(Error::InvalidJsValue("value".to_string(), "String"))?;

        let parts: Vec<&str> = s.split("-").collect();

        if parts.len() > 2 {
            return Err(Error::InvalidValue(s, "Placement"));
        }

        let side = Side::try_from_str(parts[0])?;

        if parts.len() == 1 {
            return Ok(side.into());
        }

        let alignment = Alignment::try_from_str(parts[1])?;

        Ok(AlignmentPlacement { side, alignment }.into())
    }
}

impl Into<JsValue> for Placement {
    fn into(self) -> JsValue {
        match self {
            Placement::Side(side) => side.into(),
            Placement::AlignmentPlacement(ap) => ap.into(),
        }
    }
}