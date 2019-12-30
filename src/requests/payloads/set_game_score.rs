use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{Message, ChatId},
};

/// Use this method to set the score of the specified user in a game. On success, if the message was sent by the bot, returns the edited Message, otherwise returns True. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetGameScore {
    /// Unique identifier for the target chat
    chat_id: ChatId,
    /// Identifier of the sent message
    message_id: i32,
    /// User identifier
    user_id: i32,
    /// New score, must be non-negative
    score: i32,
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    disable_edit_message: Option<bool>,
}

impl Method for SetGameScore {
    type Output = Message;

    const NAME: &'static str = "setGameScoreInline";
}

impl json::Payload for SetGameScore {}

impl dynamic::Payload for SetGameScore {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SetGameScore {
    pub fn new<C>(chat_id: C, message_id: i32, user_id: i32, score: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            message_id,
            user_id,
            score,
            force: None,
            disable_edit_message: None,
        }
    }
}

impl json::Request<'_, SetGameScore> {
    pub fn chat_id<C>(mut self, val: C) -> Self
    where
        C: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.payload.message_id = val;
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }

    pub fn score(mut self, val: i32) -> Self {
        self.payload.score = val;
        self
    }

    pub fn force(mut self, val: bool) -> Self {
        self.payload.force = Some(val);
        self
    }

    pub fn disable_edit_message(mut self, val: bool) -> Self {
        self.payload.disable_edit_message = Some(val);
        self
    }
}
