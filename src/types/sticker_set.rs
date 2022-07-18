use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Sticker, StickerKind};

/// This object represents a sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#stickerset).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,

    /// Sticker set title.
    pub title: String,

    /// Sticker kind shared by all stickers in this set.
    pub kind: StickerKind,

    /// `true`, if the sticker set contains masks.
    pub contains_masks: bool,

    /// List of all set stickers.
    pub stickers: Vec<Sticker>,

    /// Sticker set thumbnail in the .WEBP or .TGS format.
    pub thumb: Option<PhotoSize>,
}

/// This allows calling [`StickerKind`]'s methods directly on [`StickerSet`].
///
/// ```no_run
/// use teloxide_core::types::StickerSet;
///
/// let sticker: StickerSet = todo!();
///
/// let _ = sticker.is_video();
/// let _ = sticker.kind.is_video();
/// ```
impl Deref for StickerSet {
    type Target = StickerKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}
