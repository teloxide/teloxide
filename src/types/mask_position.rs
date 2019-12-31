use serde::{Deserialize, Serialize};

/// This object describes the position on faces where a mask should be placed by
/// default.
///
/// [The official docs](https://core.telegram.org/bots/api#maskposition).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One
    /// of `forehead`, `eyes`, `mouth`, or `chin`.
    pub point: String,

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
