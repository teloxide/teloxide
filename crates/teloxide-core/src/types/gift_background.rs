use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb};

/// This object describes the background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    pub center_color: Rgb,
    pub edge_color: Rgb,
    pub text_color: Rgb,
}

/// Color scheme for a user's name, message replies and link previews based on
/// a unique gift.
///
/// [The official docs](https://core.telegram.org/bots/api#uniquegiftcolors).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    pub model_custom_emoji_id: CustomEmojiId,
    pub symbol_custom_emoji_id: CustomEmojiId,
    pub light_theme_main_color: Rgb,
    pub light_theme_other_colors: Vec<Rgb>,
    pub dark_theme_main_color: Rgb,
    pub dark_theme_other_colors: Vec<Rgb>,
}
