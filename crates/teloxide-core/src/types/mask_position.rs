use std::hash::Hash;

use serde::{Deserialize, Serialize};

/// This object describes the position on faces where a mask should be placed by
/// default.
///
/// [The official docs](https://core.telegram.org/bots/api#maskposition).
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One
    /// of `forehead`, `eyes`, `mouth`, or `chin`.
    pub point: MaskPoint,

    /// Shift by X-axis measured in widths of the mask scaled to the face size,
    /// from left to right. For example, choosing `-1.0` will place mask just
    /// to the left of the default mask position.
    pub x_shift: f64,

    /// Shift by Y-axis measured in heights of the mask scaled to the face
    /// size, from top to bottom. For example, `1.0` will place the mask just
    /// below the default mask position.
    pub y_shift: f64,

    /// Mask scaling coefficient. For example, `2.0` means double size.
    pub scale: f64,
}

impl PartialEq for MaskPosition {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
            && self.x_shift.total_cmp(&other.x_shift).is_eq()
            && self.y_shift.total_cmp(&other.y_shift).is_eq()
            && self.scale.total_cmp(&other.scale).is_eq()
    }
}

impl Eq for MaskPosition {}

impl Hash for MaskPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);

        // Skip values which are NaN and canonicalize negative zero (-0.0 + 0.0 == +0.0)
        if !self.x_shift.is_nan() {
            (self.x_shift + 0.0).to_bits().hash(state);
        }
        if !self.y_shift.is_nan() {
            (self.y_shift + 0.0).to_bits().hash(state);
        }
        if !self.scale.is_nan() {
            (self.scale + 0.0).to_bits().hash(state);
        }
    }
}

/// The part of the face relative to which the mask should be placed.
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum MaskPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

impl MaskPosition {
    #[must_use]
    pub const fn new(point: MaskPoint, x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self { point, x_shift, y_shift, scale }
    }

    #[must_use]
    pub const fn point(mut self, val: MaskPoint) -> Self {
        self.point = val;
        self
    }

    #[must_use]
    pub const fn x_shift(mut self, val: f64) -> Self {
        self.x_shift = val;
        self
    }

    #[must_use]
    pub const fn y_shift(mut self, val: f64) -> Self {
        self.y_shift = val;
        self
    }

    #[must_use]
    pub const fn scale(mut self, val: f64) -> Self {
        self.scale = val;
        self
    }
}
