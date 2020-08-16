use serde::{Deserialize, Serialize};

use crate::types::{Animation, MessageEntity, PhotoSize};

/// This object represents a game.
///
/// Use [@Botfather] to create and edit games, their short names will act as
/// unique identifiers.
///
/// [@Botfather]: https://t.me/botfather
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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

impl Game {
    pub fn new<S1, S2, P>(title: S1, description: S2, photo: P) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        P: Into<Vec<PhotoSize>>,
    {
        Self {
            title: title.into(),
            description: description.into(),
            photo: photo.into(),
            text: None,
            text_entities: None,
            animation: None,
        }
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = val.into();
        self
    }

    pub fn photo<P>(mut self, val: P) -> Self
    where
        P: Into<Vec<PhotoSize>>,
    {
        self.photo = val.into();
        self
    }

    pub fn text<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.text = Some(val.into());
        self
    }

    pub fn text_entities<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<MessageEntity>>,
    {
        self.text_entities = Some(val.into());
        self
    }

    pub fn animation(mut self, val: Animation) -> Self {
        self.animation = Some(val);
        self
    }
}
