use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, MaskPosition, PhotoSize};

/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    /// Metadata of the sticker file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Sticker width, in pixels.
    ///
    /// You can assume that `max(width, height) = 512`, `min(width, height) <=
    /// 512`. In other words one dimension is exactly 512 pixels and the other
    /// is at most 512 pixels.
    pub width: u16,

    /// Sticker height, in pixels.
    ///
    /// You can assume that `max(width, height) = 512`, `min(width, height) <=
    /// 512`. In other words one dimension is exactly 512 pixels and the other
    /// is at most 512 pixels.
    pub height: u16,

    /// Kind of this sticker - regular, mask or custom emoji.
    ///
    /// In other words this represent how the sticker is presented, as a big
    /// picture/video, as a mask while editing pictures or as a custom emoji in
    /// messages.
    #[serde(flatten)]
    pub kind: StickerKind,

    /// Format of this sticker - raster/`.webp`, animated/`.tgs` or
    /// video/`.webm`.
    ///
    /// In other words this represents how the sticker is encoded.
    #[serde(flatten)]
    pub format: StickerFormat,

    /// Sticker thumbnail in the `.webp` or `.jpg` format.
    pub thumb: Option<PhotoSize>,

    /// Emoji associated with the sticker.
    pub emoji: Option<String>,

    /// Name of the sticker set to which the sticker belongs.
    pub set_name: Option<String>,
}

/// Kind of a [`Sticker`] - regular, mask or custom emoji.
///
/// Dataful version of [`StickerType`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StickerKind {
    /// "Normal", raster, animated or video sticker.
    Regular {
        /// Premium animation for the sticker, if the sticker is premium.
        premium_animation: Option<FileMeta>,
    },
    /// Mask sticker.
    Mask {
        /// For mask stickers, the position where the mask should be placed.
        mask_position: MaskPosition,
    },
    /// Custom emoji sticker.
    CustomEmoji {
        /// A unique identifier of the custom emoji.
        // FIXME(waffle): newtype
        custom_emoji_id: String,
    },
}

/// Type of a [`Sticker`] - regular, mask or custom emoji.
///
/// Dataless version of [`StickerType`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sticker_type")]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    /// "Normal", raster, animated or video sticker.
    Regular,
    /// Mask sticker.
    Mask,
    /// Custom emoji sticker.
    CustomEmoji,
}

/// Format of a [`Sticker`] - regular/webp, animated/tgs or video/webm.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "StickerFormatRaw", into = "StickerFormatRaw")]
pub enum StickerFormat {
    /// "Normal", raster, `.webp` sticker.
    Raster,
    /// [Animated], `.tgs` sticker.
    ///
    /// [Animated]: https://telegram.org/blog/animated-stickers
    Animated,
    /// [Video], `.webm` sticker.
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
/// let _ = sticker.is_regular();
/// let _ = sticker.kind.is_regular();
///
/// let _ = sticker.mask_position();
/// let _ = sticker.kind.mask_position();
/// ```
impl Deref for Sticker {
    type Target = StickerKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl Sticker {
    /// Returns `true` is this is a "normal" raster sticker.
    ///
    /// Alias to [`self.format.is_raster()`].
    ///
    /// [`self.format.is_raster()`]: StickerFormat::is_raster
    #[must_use]
    pub fn is_raster(&self) -> bool {
        self.format.is_raster()
    }

    /// Returns `true` is this is an [animated] sticker.
    ///
    /// Alias to [`self.format.is_animated()`].
    ///
    /// [`self.format.is_animated()`]: StickerFormat::is_animated
    /// [animated]: https://telegram.org/blog/animated-stickers
    #[must_use]
    pub fn is_animated(&self) -> bool {
        self.format.is_animated()
    }

    /// Returns `true` is this is a [video] sticker.
    ///
    /// Alias to [`self.format.is_video()`].
    ///
    /// [`self.format.is_video()`]: StickerFormat::is_video
    /// [video]: https://telegram.org/blog/video-stickers-better-reactions
    #[must_use]
    pub fn is_video(&self) -> bool {
        self.format.is_video()
    }
}

impl StickerKind {
    /// Converts [`StickerKind`] to [`StickerType`]
    #[must_use]
    pub fn type_(&self) -> StickerType {
        match self {
            StickerKind::Regular { .. } => StickerType::Regular,
            StickerKind::Mask { .. } => StickerType::Mask,
            StickerKind::CustomEmoji { .. } => StickerType::CustomEmoji,
        }
    }

    /// Returns `true` if the sticker kind is [`Regular`].
    ///
    /// [`Regular`]: StickerKind::Regular
    #[must_use]
    pub fn is_regular(&self) -> bool {
        self.type_().is_regular()
    }

    /// Returns `true` if the sticker kind is [`Mask`].
    ///
    /// [`Mask`]: StickerKind::Mask
    #[must_use]
    pub fn is_mask(&self) -> bool {
        self.type_().is_mask()
    }

    /// Returns `true` if the sticker kind is [`CustomEmoji`].
    ///
    /// [`CustomEmoji`]: StickerKind::CustomEmoji
    #[must_use]
    pub fn is_custom_emoji(&self) -> bool {
        self.type_().is_custom_emoji()
    }

    /// Getter for [`StickerKind::Regular::premium_animation`].
    #[must_use]
    pub fn premium_animation(&self) -> Option<&FileMeta> {
        if let Self::Regular { premium_animation } = self {
            premium_animation.as_ref()
        } else {
            None
        }
    }

    /// Getter for [`StickerKind::Mask::mask_position`].
    #[must_use]
    pub fn mask_position(&self) -> Option<MaskPosition> {
        if let Self::Mask { mask_position } = self {
            Some(*mask_position)
        } else {
            None
        }
    }

    /// Getter for [`StickerKind::CustomEmoji::custom_emoji_id`].
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        if let Self::CustomEmoji { custom_emoji_id } = self {
            Some(custom_emoji_id)
        } else {
            None
        }
    }
}

impl StickerType {
    /// Returns `true` if the sticker type is [`Regular`].
    ///
    /// [`Regular`]: StickerType::Regular
    #[must_use]
    pub fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Returns `true` if the sticker type is [`Mask`].
    ///
    /// [`Mask`]: StickerType::Mask
    #[must_use]
    pub fn is_mask(&self) -> bool {
        matches!(self, Self::Mask)
    }

    /// Returns `true` if the sticker type is [`CustomEmoji`].
    ///
    /// [`CustomEmoji`]: StickerType::CustomEmoji
    #[must_use]
    pub fn is_custom_emoji(&self) -> bool {
        matches!(self, Self::CustomEmoji)
    }
}

impl StickerFormat {
    /// Returns `true` if the sticker format is [`Raster`].
    ///
    /// [`Raster`]: StickerFormat::Raster
    #[must_use]
    pub fn is_raster(&self) -> bool {
        matches!(self, Self::Raster)
    }

    /// Returns `true` if the sticker format is [`Animated`].
    ///
    /// [`Animated`]: StickerFormat::Animated
    #[must_use]
    pub fn is_animated(&self) -> bool {
        matches!(self, Self::Animated)
    }

    /// Returns `true` if the sticker format is [`Video`].
    ///
    /// [`Video`]: StickerFormat::Video
    #[must_use]
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video)
    }
}

#[derive(Serialize, Deserialize)]
struct StickerFormatRaw {
    is_animated: bool,
    is_video: bool,
}

impl TryFrom<StickerFormatRaw> for StickerFormat {
    type Error = &'static str;

    fn try_from(
        StickerFormatRaw { is_animated, is_video }: StickerFormatRaw,
    ) -> Result<Self, Self::Error> {
        let ret = match (is_animated, is_video) {
            (false, false) => Self::Raster,
            (true, false) => Self::Animated,
            (false, true) => Self::Video,
            (true, true) => return Err("`is_animated` and `is_video` present at the same time"),
        };

        Ok(ret)
    }
}

impl From<StickerFormat> for StickerFormatRaw {
    fn from(kind: StickerFormat) -> Self {
        match kind {
            StickerFormat::Raster => Self { is_animated: false, is_video: false },
            StickerFormat::Animated => Self { is_animated: true, is_video: false },
            StickerFormat::Video => Self { is_animated: false, is_video: true },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{MaskPoint, Sticker, StickerFormat, StickerType};

    #[test]
    fn mask_serde() {
        // Taken from a real (mask) sticker set
        let json = r#"{
            "width": 512,
            "height": 512,
            "emoji": "🎭",
            "set_name": "Coronamask",
            "is_animated": false,
            "is_video": false,
            "type": "mask",
            "mask_position": {
                "point": "forehead",
                "x_shift": -0.0125,
                "y_shift": 0.5525,
                "scale": 1.94
            },
            "thumb": {
                "file_id": "AAMCAQADFQABYzA0qlYHijpjMzMwBFKnEVE5XdkAAjIKAAK_jJAE1TRw7D936M8BAAdtAAMpBA",
                "file_unique_id": "AQADMgoAAr-MkARy",
                "file_size": 11028,
                "width": 320,
                "height": 320
            },
            "file_id": "CAACAgEAAxUAAWMwNKpWB4o6YzMzMARSpxFROV3ZAAIyCgACv4yQBNU0cOw_d-jPKQQ",
            "file_unique_id": "AgADMgoAAr-MkAQ",
            "file_size": 18290
        }"#;

        let sticker: Sticker = serde_json::from_str(json).unwrap();

        // Assert some basic properties are correctly deserialized
        assert_eq!(sticker.type_(), StickerType::Mask);
        assert_eq!(sticker.mask_position().unwrap().point, MaskPoint::Forehead);
        assert_eq!(sticker.is_animated(), false);
        assert_eq!(sticker.is_video(), false);
        assert_eq!(sticker.thumb.clone().unwrap().file.size, 11028);
        assert_eq!(sticker.file.size, 18290);
        assert_eq!(sticker.width, 512);
        assert_eq!(sticker.height, 512);

        let json2 = serde_json::to_string(&sticker).unwrap();
        let sticker2: Sticker = serde_json::from_str(&json2).unwrap();
        assert_eq!(sticker, sticker2);
    }

    #[test]
    fn regular_serde() {
        // Taken from a real sticker set
        let json = r#"{
            "width": 463,
            "height": 512,
            "emoji": "🍿",
            "set_name": "menhera2",
            "is_animated": false,
            "is_video": false,
            "type": "regular",
            "thumb": {
                "file_id": "AAMCAgADFQABYzBxOJ1GWrttqL7FSRwdAtrq-AkAAtkHAALBGJ4LUUUh5CUew90BAAdtAAMpBA",
                "file_unique_id": "AQAD2QcAAsEYngty",
                "file_size": 4558,
                "width": 116,
                "height": 128
            },
            "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
            "file_unique_id": "AgAD2QcAAsEYngs",
            "file_size": 25734
        }"#;

        let sticker: Sticker = serde_json::from_str(json).unwrap();

        // Assert some basic properties are correctly deserialized
        assert_eq!(sticker.type_(), StickerType::Regular);
        assert_eq!(sticker.premium_animation(), None);
        assert_eq!(sticker.is_animated(), false);
        assert_eq!(sticker.is_video(), false);
        assert_eq!(sticker.thumb.clone().unwrap().file.size, 4558);
        assert_eq!(sticker.file.size, 25734);
        assert_eq!(sticker.width, 463);
        assert_eq!(sticker.height, 512);
        assert_eq!(sticker.set_name.as_deref(), Some("menhera2"));

        let json2 = serde_json::to_string(&sticker).unwrap();
        let sticker2: Sticker = serde_json::from_str(&json2).unwrap();
        assert_eq!(sticker, sticker2);
    }

    #[test]
    fn sticker_format_serde() {
        {
            let json = r#"{"is_animated":false,"is_video":false}"#;
            let fmt: StickerFormat = serde_json::from_str(json).unwrap();
            assert_eq!(fmt, StickerFormat::Raster);

            let json2 = serde_json::to_string(&fmt).unwrap();
            assert_eq!(json, json2);
        }
        {
            let json = r#"{"is_animated":true,"is_video":false}"#;
            let fmt: StickerFormat = serde_json::from_str(json).unwrap();
            assert_eq!(fmt, StickerFormat::Animated);

            let json2 = serde_json::to_string(&fmt).unwrap();
            assert_eq!(json, json2);
        }
        {
            let json = r#"{"is_animated":false,"is_video":true}"#;
            let fmt: StickerFormat = serde_json::from_str(json).unwrap();
            assert_eq!(fmt, StickerFormat::Video);

            let json2 = serde_json::to_string(&fmt).unwrap();
            assert_eq!(json, json2);
        }
        {
            let json = r#"{"is_animated":true,"is_video":true}"#;
            let fmt: Result<StickerFormat, _> = serde_json::from_str(json);
            assert!(fmt.is_err());
        }
    }
}
