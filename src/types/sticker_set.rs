use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Sticker};

/// This object represents a sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#stickerset).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,

    /// Sticker set title.
    pub title: String,

    /// `true`, if the sticker set contains [animated stickers].
    ///
    /// [animates stickers]: https://telegram.org/blog/animated-stickers
    pub is_animated: bool,

    /// `true`, if the sticker set contains masks.
    pub contains_masks: bool,

    /// List of all set stickers.
    pub stickers: Vec<Sticker>,

    /// Sticker set thumbnail in the .WEBP or .TGS format.
    thumb: Option<PhotoSize>,
}

impl StickerSet {
    pub fn new<S1, S2, St>(
        name: S1,
        title: S2,
        is_animated: bool,
        contains_masks: bool,
        stickers: St,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        St: Into<Vec<Sticker>>,
    {
        Self {
            name: name.into(),
            title: title.into(),
            is_animated,
            contains_masks,
            stickers: stickers.into(),
            thumb: None,
        }
    }

    pub fn name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.name = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_animated(mut self, val: bool) -> Self {
        self.is_animated = val;
        self
    }

    pub fn contains_masks(mut self, val: bool) -> Self {
        self.contains_masks = val;
        self
    }

    pub fn stickers<S>(mut self, val: S) -> Self
    where
        S: Into<Vec<Sticker>>,
    {
        self.stickers = val.into();
        self
    }
}
