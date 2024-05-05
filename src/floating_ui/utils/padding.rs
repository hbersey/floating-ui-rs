use super::PartialSideObject;

pub enum Padding {
    Number(f64),
    PartialSideObject(PartialSideObject),
}

impl From<f64> for Padding {
    fn from(child: f64) -> Self {
        Self::Number(child)
    }
}

impl TryInto<f64> for Padding {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Self::Number(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl From<PartialSideObject> for Padding {
    fn from(child: PartialSideObject) -> Self {
        Self::PartialSideObject(child)
    }
}

impl TryInto<PartialSideObject> for Padding {
    type Error = ();

    fn try_into(self) -> Result<PartialSideObject, Self::Error> {
        match self {
            Self::PartialSideObject(value) => Ok(value),
            _ => Err(()),
        }
    }
}


