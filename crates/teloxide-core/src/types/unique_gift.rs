use crate::types::{Chat, Rgb, Sticker};
use serde::{Deserialize, Serialize};

/// This object describes a unique gift that was upgraded from a regular gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct UniqueGift {
    /// Human-readable name of the regular gift from which this unique gift was
    /// upgraded
    pub base_name: String,

    /// Unique name of the gift. This name can be used in `https://t.me/nft/...` links and story areas
    pub name: String,

    /// Unique number of the upgraded gift among gifts upgraded from the same
    /// regular gift
    pub number: u64,

    /// Model of the gift
    pub model: UniqueGiftModel,

    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,

    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,

    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}

/// This object describes the model of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct UniqueGiftModel {
    /// Name of the model
    pub name: String,

    /// The sticker that represents the unique gift
    pub sticker: Sticker,

    /// The number of unique gifts that receive this model for every 1000 gifts
    /// upgraded
    pub rarity_per_mille: u32,
}

/// This object describes the symbol shown on the pattern of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol
    pub name: String,

    /// The sticker that represents the unique gift
    pub sticker: Sticker,

    /// The number of unique gifts that receive this model for every 1000 gifts
    /// upgraded
    pub rarity_per_mille: u32,
}

/// This object describes the backdrop of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: String,

    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,

    /// The number of unique gifts that receive this backdrop for every 1000
    /// gifts upgraded
    pub rarity_per_mille: u32,
}

/// This object describes the colors of the backdrop of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: Rgb,

    /// The color on the edges of the backdrop in RGB format
    pub edge_color: Rgb,

    /// The color to be applied to the symbol in RGB format
    pub symbol_color: Rgb,

    /// The color for the text on the backdrop in RGB format
    pub text_color: Rgb,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::{FileMeta, StickerFormatFlags, StickerKind};

    #[test]
    fn deserialize() {
        let sticker = Sticker {
            file: FileMeta {
                id: "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ".into(),
                unique_id: "AgAD2QcAAsEYngs".into(),
                size: 25734,
            },
            width: 463,
            height: 512,
            kind: StickerKind::Regular { premium_animation: None },
            flags: StickerFormatFlags { is_animated: false, is_video: false },
            thumbnail: None,
            emoji: None,
            set_name: None,
            needs_repainting: false,
        };

        let unique_gift = UniqueGift {
            base_name: "name".to_owned(),
            name: "name".to_owned(),
            number: 123,
            model: UniqueGiftModel {
                name: "name".to_owned(),
                sticker: sticker.clone(),
                rarity_per_mille: 123,
            },
            symbol: UniqueGiftSymbol {
                name: "name".to_owned(),
                sticker: sticker.clone(),
                rarity_per_mille: 123,
            },
            backdrop: UniqueGiftBackdrop {
                name: "name".to_owned(),
                colors: UniqueGiftBackdropColors {
                    center_color: Rgb { r: 255, g: 255, b: 0 },
                    edge_color: Rgb { r: 255, g: 255, b: 0 },
                    symbol_color: Rgb { r: 255, g: 255, b: 0 },
                    text_color: Rgb { r: 255, g: 255, b: 0 },
                },
                rarity_per_mille: 123,
            },
            publisher_chat: None,
        };

        let unique_gift_json = r#"{
            "base_name": "name",
            "name": "name",
            "number": 123,
            "model": {
                "name": "name",
                "sticker": {
                    "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                    "file_unique_id": "AgAD2QcAAsEYngs",
                    "file_size": 25734,
                    "width": 463,
                    "height": 512,
                    "type": "regular",
                    "premium_animation": null,
                    "is_animated": false,
                    "is_video": false,
                    "needs_repainting": false
                },
                "rarity_per_mille": 123
            },
            "symbol": {
                "name": "name",
                "sticker": {
                    "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                    "file_unique_id": "AgAD2QcAAsEYngs",
                    "file_size": 25734,
                    "width": 463,
                    "height": 512,
                    "type": "regular",
                    "premium_animation": null,
                    "is_animated": false,
                    "is_video": false,
                    "needs_repainting": false
                },
                "rarity_per_mille": 123
            },
            "backdrop": {
                "name": "name",
                "colors": {
                    "center_color": 16776960,
                    "edge_color": 16776960,
                    "symbol_color": 16776960,
                    "text_color": 16776960
                },
                "rarity_per_mille": 123
            }
        }"#;

        assert_eq!(serde_json::from_str::<UniqueGift>(unique_gift_json).unwrap(), unique_gift);
    }
}
