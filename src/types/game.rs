use serde::{Deserialize, Serialize};

use crate::types::{Animation, MessageEntity, PhotoSize};

/// This object represents a game. Use [@Botfather] to create and edit games,
/// their short names will act as unique identifiers.
///
/// [@Botfather]: https://t.me/botfather
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Game {
    /// Title of the game.
    pub title: String,

    /// Description of the game.
    pub description: String,

    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,

    /// Brief description of the game or high scores included in the game
    /// message. Can be automatically edited to include current high scores
    /// for the game when the bot calls [`Bot::set_game_score`], or manually
    /// edited using [`Bot::edit_message_text`]. 0-4096 characters.
    ///
    /// [`Bot::set_game_score`]: crate::Bot::set_game_score
    ///
    /// [`Bot::edit_message_text`]: crate::Bot::edit_message_text
    pub text: Option<String>,

    /// Special entities that appear in text, such as usernames, URLs, bot
    /// commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Animation that will be displayed in the game message in chats. Upload
    /// via [@Botfather].
    ///
    /// [@Botfather]: https://t.me/botfather
    pub animation: Option<Animation>,
}
