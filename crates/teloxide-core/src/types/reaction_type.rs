use serde::Serialize;

use crate::types::ReactionEmoji;

/// Sticker file that may be uploaded to telegram.
#[derive(Clone, Debug, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ReactionType {
    /// The reaction is based on an emoji.
    Emoji {
        /// Reaction emoji.
        emoji: ReactionEmoji
    },

    /// The reaction is based on an emoji.
    CustomEmoji {
        /// Custom emoji identifier.
        #[serde(rename = "custom_emoji_id")]
        id: String
    }
}

impl ReactionType {
    /// The reaction is based on an emoji.
    #[must_use]
    pub fn emoji(emoji: ReactionEmoji) -> Self {
        ReactionType::Emoji { emoji }
    }

    /// The reaction is based on an emoji.
    #[must_use]
    pub fn custom_emoji(id: String) -> Self {
        ReactionType::CustomEmoji { id }
    }
}