use crate::core::types::{Message, User};

/// This object represents an incoming callback query from a callback button in
/// an inline keyboard.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query.
    /// Note that message content and message date will not be available if the
    /// message is too old
    pub message: Message,
    /// Global identifier, uniquely corresponding to the chat to which the
    /// message with the callback button was sent. Useful for high scores
    /// in games.
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that a bad client
    /// can send arbitrary data in this field.
    pub data: String,
}
