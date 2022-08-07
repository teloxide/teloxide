use std::{convert::TryFrom, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, MaskPosition, PhotoSize};

/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    /// Kind of this sticker - webp, animated or video.
    #[serde(flatten)]
    pub kind: StickerKind,

    /// Sticker thumbnail in the .webp or .jpg format.
    pub thumb: Option<PhotoSize>,

    /// Emoji associated with the sticker.
    pub emoji: Option<String>,

    /// Name of the sticker set to which the sticker belongs.
    pub set_name: Option<String>,

    /// Premium animation for the sticker, if the sticker is premium.
    pub premium_animation: Option<FileMeta>,

    /// For mask stickers, the position where the mask should be placed.
    pub mask_position: Option<MaskPosition>,

    /// File size in bytes.
    #[serde(default = "crate::types::file::file_size_fallback")]
    pub file_size: u32,
}

/// Kind of a sticker - webp, animated or video.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "StickerKindRaw", into = "StickerKindRaw")]
pub enum StickerKind {
    /// "Normal", raster sticker.
    Webp,
    /// [Animated] sticker.
    ///
    /// [Animated]: https://telegram.org/blog/animated-stickers
    Animated,
    /// [Video] sticker.
    ///
    /// [Video]: https://telegram.org/blog/video-stickers-better-reactions
    Video,
}

/// This allows calling [`StickerKind`]'s methods directly on [`Sticker`].
///
/// ```no_run
/// use teloxide_core::types::Sticker;
///
/// let sticker: Sticker = todo!();
///
/// let _ = sticker.is_video();
/// let _ = sticker.kind.is_video();
/// ```
impl Deref for Sticker {
    type Target = StickerKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl StickerKind {
    /// Returns `true` is this is a "normal" raster sticker.
    #[must_use]
    pub fn is_webp(&self) -> bool {
        matches!(self, Self::Webp)
    }

    /// Returns `true` is this is an [animated] sticker.
    ///
    /// [animated]: https://telegram.org/blog/animated-stickers
    #[must_use]
    pub fn is_animated(&self) -> bool {
        matches!(self, Self::Animated)
    }

    /// Returns `true` is this is a [video] sticker.
    ///
    /// [video]: https://telegram.org/blog/video-stickers-better-reactions
    #[must_use]
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video)
    }
}

#[derive(Serialize, Deserialize)]
struct StickerKindRaw {
    is_animated: bool,
    is_video: bool,
}

impl TryFrom<StickerKindRaw> for StickerKind {
    type Error = &'static str;

    fn try_from(
        StickerKindRaw {
            is_animated,
            is_video,
        }: StickerKindRaw,
    ) -> Result<Self, Self::Error> {
        let ret = match (is_animated, is_video) {
            (false, false) => Self::Webp,
            (true, false) => Self::Animated,
            (false, true) => Self::Video,
            (true, true) => return Err("`is_animated` and `is_video` present at the same time"),
        };

        Ok(ret)
    }
}

impl From<StickerKind> for StickerKindRaw {
    fn from(kind: StickerKind) -> Self {
        match kind {
            StickerKind::Webp => Self {
                is_animated: false,
                is_video: false,
            },
            StickerKind::Animated => Self {
                is_animated: true,
                is_video: false,
            },
            StickerKind::Video => Self {
                is_animated: false,
                is_video: true,
            },
        }
    }
}
