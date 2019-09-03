use serde::Deserialize;

use crate::core::types::Sticker;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Sticker {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
}
