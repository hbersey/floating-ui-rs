use super::{Axis, Coords, Dimensions, Length, Rect, Side, SideObject};
use crate::utils_::{extends, index};

pub struct ClientRectObject {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

extends!(ClientRectObject => Rect, x, y, width, height);
extends!(ClientRectObject => Coords, x, y);
extends!(ClientRectObject => Dimensions, width, height);
index!(ClientRectObject, Axis { X, Y } => f64);
index!(ClientRectObject, Length { Width, Height } => f64);

extends!(ClientRectObject => SideObject, top, right, bottom, left);
index!(ClientRectObject, Side { Top, Right, Bottom, Left } => f64);
