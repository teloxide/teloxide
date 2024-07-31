use serde::{Deserialize, Serialize};

use crate::types::MessageEntity;

/// This object contains information about the quoted part of a message that is
/// replied to by the given message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given
    /// message
    pub text: String,
    /// Special entities that appear in the quote. Currently, only _bold_,
    /// _italic_, _underline_, _strikethrough_, _spoiler_, and
    /// _custom_emoji_ entities are kept in quotes.
    #[serde(default)]
    pub entities: Vec<MessageEntity>,
    /// Approximate quote position in the original message in UTF-16 code units
    /// as specified by the sender
    pub position: u32,
    /// True, if the quote was chosen manually by the message sender. Otherwise,
    /// the quote was added automatically by the server.
    #[serde(default)]
    pub is_manual: bool,
}
