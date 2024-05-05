use wasm_bindgen::JsValue;

use crate::utils_::StringUnion as _;

use super::{Alignment, Side};

#[derive(Clone)]
pub struct AlignmentPlacement {
    pub side: Side,
    pub alignment: Alignment,
}

impl ToString for AlignmentPlacement {
    fn to_string(&self) -> String {
        format!("{}-{}", self.side.as_str(), self.alignment.as_str())
    }
}

impl TryFrom<String> for AlignmentPlacement {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(' ').collect();
        if parts.len() != 2 {
            return Err(());
        }

        let side = match Side::try_from_str(parts[0]) {
            Ok(side) => side,
            Err(_) => {
                return Err(());
            }
        };

        let alignment = match Alignment::try_from_str(parts[1]) {
            Ok(alignment) => alignment,
            Err(_) => {
                return Err(());
            }
        };

        Ok(Self { side, alignment })
    }
}

impl Into<JsValue> for AlignmentPlacement {
    fn into(self) -> JsValue {
        self.to_string().into()
    }
}

impl TryFrom<JsValue> for AlignmentPlacement {
    type Error = ();

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let s = match value.as_string() {
            Some(s) => s,
            None => {
                return Err(());
            }
        };

        Self::try_from(s)
    }
}
