use serde::{Deserialize, Serialize};

use crate::types::{Animation, MessageEntity, PhotoSize, User};

/// This object represents a game.
///
/// Use [@Botfather] to create and edit games, their short names will act as
/// unique identifiers.
///
/// [@Botfather]: https://t.me/botfather
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Game {
    /// Title of the game.
    pub title: String,

    /// Description of the game.
    pub description: String,

    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,

    /// Brief description of the game or high scores included in the game
    /// message. Can be automatically edited to include current high scores
    /// for the game when the bot calls [`SetGameScore`], or manually
    /// edited using [`EditMessageText`]. 0-4096 characters.
    ///
    /// [`SetGameScore`]: crate::payloads::SetGameScore
    /// [`EditMessageText`]: crate::payloads::EditMessageText
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

impl Game {
    /// Returns all users that are "contained" in this `Game`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use crate::util::{flatten, mentioned_users_from_entities};

        flatten(self.text_entities.as_deref().map(mentioned_users_from_entities))
    }
}
