use crate::types::Sticker;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Sticker {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
}
