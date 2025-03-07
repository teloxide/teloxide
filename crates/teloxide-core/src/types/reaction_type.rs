use serde::{Deserialize, Serialize};

/// The reaction type is based on an emoji or custom emoji.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ReactionType {
    /// Emoji reaction.
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
    /// Custom emoji reaction.
    CustomEmoji {
        /// Custom emoji identifier.
        custom_emoji_id: String,
    },
    /// Paid reaction.
    Paid,
}

impl ReactionType {
    #[must_use]
    pub fn emoji(&self) -> Option<&String> {
        match &self {
            Self::Emoji { emoji } => Some(emoji),
            _ => None,
        }
    }

    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&String> {
        match &self {
            Self::CustomEmoji { custom_emoji_id } => Some(custom_emoji_id),
            _ => None,
        }
    }
}
