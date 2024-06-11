use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Sticker, StickerFormat, StickerFormatFlags, StickerType};

/// This object represents a sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#stickerset).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,

    /// Sticker set title.
    pub title: String,

    /// Sticker type shared by all stickers in this set.
    #[serde(flatten)]
    pub kind: StickerType,

    // FIXME: remove it in 7.2 https://core.telegram.org/bots/api#march-31-2024
    /// Sticker format flags shared by all stickers in this set.
    #[serde(flatten)]
    pub flags: StickerFormatFlags,

    /// List of all set stickers.
    pub stickers: Vec<Sticker>,

    /// Sticker set thumbnail in the `.webp`, `.tgs` or `.webm` format.
    pub thumbnail: Option<PhotoSize>,
}

/// This allows calling [`StickerType`]'s methods directly on [`StickerSet`].
///
/// ```no_run
/// use teloxide_core::types::StickerSet;
///
/// let sticker: StickerSet = todo!();
///
/// let _ = sticker.is_mask();
/// let _ = sticker.kind.is_mask();
/// ```
impl Deref for StickerSet {
    type Target = StickerType;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl StickerSet {
    // FIXME: remove deprecation, when it will be a way to determine the format of
    // the sticker set
    /// Returns the format of the stickers in this set
    /// 
    /// Note: this method currently is useless, so the format is always StickerFormat::Static
    #[must_use]
    #[deprecated(note = "TBA7.2 brought the breaking change: flags 'is_video' and 'is_animated' \
                         were removed, so currently there is no way to determine the format of \
                         the sticker set in the currently supported version (TBA6.6)")]
    pub fn format(&self) -> StickerFormat {
        self.flags.format()
    }

    /// Returns `true` is this is a "normal" raster sticker.
    ///
    /// Alias to [`self.format().is_static()`].
    ///
    /// [`self.format().is_static()`]: StickerFormat::is_static
    #[must_use]
    #[deprecated(note = "TBA7.2 brought the breaking change: flags 'is_video' and 'is_animated' \
                         were removed, so currently there is no way to determine the format of \
                         the sticker set in the currently supported version (TBA6.6)")]
    pub fn is_static(&self) -> bool {
        #[allow(deprecated)]
        self.format().is_static()
    }

    /// Returns `true` is this is an [animated] sticker.
    ///
    /// Alias to [`self.format().is_animated()`].
    ///
    /// [`self.format().is_animated()`]: StickerFormat::is_animated
    /// [animated]: https://telegram.org/blog/animated-stickers
    #[must_use]
    #[deprecated(note = "TBA7.2 brought the breaking change: flags 'is_video' and 'is_animated' \
                         were removed, so currently there is no way to determine the format of \
                         the sticker set in the currently supported version (TBA6.6)")]
    pub fn is_animated(&self) -> bool {
        #[allow(deprecated)]
        self.format().is_animated()
    }

    /// Returns `true` is this is a [video] sticker.
    ///
    /// Alias to [`self.format().is_video()`].
    ///
    /// [`self.format().is_video()`]: StickerFormat::is_video
    /// [video]: https://telegram.org/blog/video-stickers-better-reactions
    #[must_use]
    #[deprecated(note = "TBA7.2 brought the breaking change: flags 'is_video' and 'is_animated' \
                         were removed, so currently there is no way to determine the format of \
                         the sticker set in the currently supported version (TBA6.6)")]
    pub fn is_video(&self) -> bool {
        #[allow(deprecated)]
        self.format().is_video()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::StickerSet;

    #[test]
    fn smoke_serde() {
        // https://t.me/addstickers/teloxide_test
        let json = r#"{
            "name": "teloxide_test",
            "title": "teloxide-test",
            "is_animated": false,
            "is_video": false,
            "sticker_type": "regular",
            "contains_masks": false,
            "stickers": [
                {
                    "width": 512,
                    "height": 512,
                    "emoji": "⚙️",
                    "set_name": "teloxide_test",
                    "is_animated": false,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAQADFQABYzB4ATH0sqXx351gZ5GpY1Z3Tl8AAlgCAAJ1t4hFbxNCoAg1-akBAAdtAAMpBA",
                        "file_unique_id": "AQADWAIAAnW3iEVy",
                        "file_size": 7698,
                        "width": 320,
                        "height": 320
                    },
                    "file_id": "CAACAgEAAxUAAWMweAEx9LKl8d-dYGeRqWNWd05fAAJYAgACdbeIRW8TQqAINfmpKQQ",
                    "file_unique_id": "AgADWAIAAnW3iEU",
                    "file_size": 12266
                },
                {
                    "width": 512,
                    "height": 512,
                    "emoji": "⚙️",
                    "set_name": "teloxide_test",
                    "is_animated": false,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAQADFQABYzB4AcABR8-MuvGagis9Pk6liSAAAs8DAAL2YYBFNbvduoN1p7oBAAdtAAMpBA",
                        "file_unique_id": "AQADzwMAAvZhgEVy",
                        "file_size": 7780,
                        "width": 320,
                        "height": 320
                    },
                    "file_id": "CAACAgEAAxUAAWMweAHAAUfPjLrxmoIrPT5OpYkgAALPAwAC9mGARTW73bqDdae6KQQ",
                    "file_unique_id": "AgADzwMAAvZhgEU",
                    "file_size": 12158
                }
            ]
        }"#;

        let set: StickerSet = serde_json::from_str(json).unwrap();

        assert!(set.is_regular());
        assert!(set.thumbnail.is_none());
        assert_eq!(set.stickers.len(), 2);
    }

    #[test]
    // In TBA7.2 fields `is_video` and `is_audio` are removed, so it's the breaking
    // change (previously, sticker set format inference heavily relied upon these
    // fields)
    fn test() {
        let json = r#"{
            "name": "tba66_by_memorization_helper_bot",
            "title": "Teloxide TBA6.6 TEST",
            "sticker_type": "regular",
            "contains_masks": false,
            "stickers": [
                {
                    "width": 512,
                    "height": 512,
                    "emoji": "\\ud83e\\udd80",
                    "set_name": "tba66_by_memorization_helper_bot",
                    "is_animated": false,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAgADFQABZmbS1r0a5NWqrPIHJSCabGw3LUwAAh5MAAKRGTlL-H1XHcgw5coBAAdtAAM1BA",
                        "file_unique_id": "AQADHkwAApEZOUty",
                        "file_size": 7786,
                        "width": 320,
                        "height": 320
                    },
                    "thumb": {
                        "file_id": "AAMCAgADFQABZmbS1r0a5NWqrPIHJSCabGw3LUwAAh5MAAKRGTlL-H1XHcgw5coBAAdtAAM1BA",
                        "file_unique_id": "AQADHkwAApEZOUty",
                        "file_size": 7786,
                        "width": 320,
                        "height": 320
                    },
                    "file_id": "CAACAgIAAxUAAWZm0ta9GuTVqqzyByUgmmxsNy1MAAIeTAACkRk5S_h9Vx3IMOXKNQQ",
                    "file_unique_id": "AgADHkwAApEZOUs",
                    "file_size": 11936
                },
                {
                    "width": 512,
                    "height": 512,
                    "emoji": "\\ud83e\\udd80",
                    "set_name": "tba66_by_memorization_helper_bot",
                    "is_animated": false,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAgADFQABZmbS1jN5O3rMp4gsH3eeBPajWVUAAoxIAAKtXDhLhabEKw0iE9sBAAdtAAM1BA",
                        "file_unique_id": "AQADjEgAAq1cOEty",
                        "file_size": 7768,
                        "width": 320,
                        "height": 320
                    },
                    "thumb": {
                        "file_id": "AAMCAgADFQABZmbS1jN5O3rMp4gsH3eeBPajWVUAAoxIAAKtXDhLhabEKw0iE9sBAAdtAAM1BA",
                        "file_unique_id": "AQADjEgAAq1cOEty",
                        "file_size": 7768,
                        "width": 320,
                        "height": 320
                    },
                    "file_id": "CAACAgIAAxUAAWZm0tYzeTt6zKeILB93ngT2o1lVAAKMSAACrVw4S4WmxCsNIhPbNQQ",
                    "file_unique_id": "AgADjEgAAq1cOEs",
                    "file_size": 12092
                }
            ]
        }"#;

        let set: StickerSet = serde_json::from_str(json).unwrap();

        assert!(set.is_regular());
        assert!(set.thumbnail.is_none());
        assert_eq!(set.stickers.len(), 2);
    }
}
