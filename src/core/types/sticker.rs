use crate::core::types::{MaskPosition, PhotoSize};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sticker {
    pub file_id: String,
    pub width: u16,
    pub height: u16,
    pub is_animated: bool,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<u32>,
}
