use serde::{Deserialize, Serialize};

use crate::types::{MaskPosition, PhotoSize};

/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl Sticker {
    pub fn new<S1, S2>(
        file_id: S1,
        file_unique_id: S2,
        width: u16,
        height: u16,
        is_animated: bool,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width,
            height,
            is_animated,
            thumb: None,
            emoji: None,
            set_name: None,
            mask_position: None,
            file_size: None,
        }
    }

    pub fn file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_id = val.into();
        self
    }

    pub fn file_unique_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_unique_id = val.into();
        self
    }

    pub fn height(mut self, val: u16) -> Self {
        self.height = val;
        self
    }

    pub fn width(mut self, val: u16) -> Self {
        self.width = val;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_animated(mut self, val: bool) -> Self {
        self.is_animated = val;
        self
    }

    pub fn thumb(mut self, val: PhotoSize) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn emoji<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.emoji = Some(val.into());
        self
    }

    pub fn set_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.set_name = Some(val.into());
        self
    }

    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }

    pub fn file_size(mut self, val: u32) -> Self {
        self.file_size = Some(val);
        self
    }
}
