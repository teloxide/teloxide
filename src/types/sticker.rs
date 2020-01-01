use serde::{Deserialize, Serialize};

use crate::types::{MaskPosition, PhotoSize};

/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Sticker {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Sticker width.
    pub width: u16,

    /// Sticker height.
    pub height: u16,

    /// `true`, if the sticker is [animated].
    ///
    /// [animated]: https://telegram.org/blog/animated-stickers
    pub is_animated: bool,

    /// Sticker thumbnail in the .webp or .jpg format.
    pub thumb: Option<PhotoSize>,

    /// Emoji associated with the sticker.
    pub emoji: Option<String>,

    /// Name of the sticker set to which the sticker belongs.
    pub set_name: Option<String>,

    /// For mask stickers, the position where the mask should be placed.
    pub mask_position: Option<MaskPosition>,

    /// File size.
    pub file_size: Option<u32>,
}
