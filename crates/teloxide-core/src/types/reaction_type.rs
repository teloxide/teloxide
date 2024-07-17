use serde::{Deserialize, Serialize};

/// The reaction type is based on an emoji.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReactionType {
    /// Kind of this reaction type - emoji or custom emoji.
    #[serde(flatten)]
    pub kind: ReactionTypeKind,
}

/// Kind of a [`ReactionType`] - emoji or custom emoji.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ReactionTypeKind {
    /// "emoji" or "custom_emoji" reaction
    Emoji {
        /// Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥",
        /// "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩",
        /// "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳",
        /// "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨",
        /// "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻",
        /// "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡",
        /// "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘",
        /// "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
        emoji: String,
    },
    /// Custom emoji sticker.
    CustomEmoji {
        /// Custom emoji identifier.
        custom_emoji_id: String,
    },
}

impl ReactionType {
    #[must_use]
    pub fn emoji(&self) -> Option<&String> {
        match &self.kind {
            ReactionTypeKind::Emoji { emoji } => Some(emoji),
            _ => None,
        }
    }

    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&String> {
        match &self.kind {
            ReactionTypeKind::CustomEmoji { custom_emoji_id } => Some(custom_emoji_id),
            _ => None,
        }
    }
}
