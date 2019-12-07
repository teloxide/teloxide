#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}
