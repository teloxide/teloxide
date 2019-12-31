use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{InlineKeyboardMarkup, Message},
};

/// Use this method to send a game. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendGame {
    /// Unique identifier for the target chat
    chat_id: i32,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
    game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// A JSON-serialized object for an inline keyboard. If empty, one ‘Play game_title’ button will be shown. If not empty, the first button must launch the game.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for SendGame {
    type Output = Message;

    const NAME: &'static str = "sendGame";
}

impl json::Payload for SendGame {}

impl dynamic::Payload for SendGame {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SendGame {
    pub fn new<G>(chat_id: i32, game_short_name: G) -> Self
    where
        G: Into<String>
    {
        let game_short_name = game_short_name.into();
        Self {
            chat_id,
            game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, SendGame> {
    pub fn chat_id(mut self, val: i32) -> Self {
        self.payload.chat_id = val;
        self
    }

    pub fn game_short_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.game_short_name = val.into();
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.payload.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.payload.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
                 