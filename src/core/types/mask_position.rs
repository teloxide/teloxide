#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

