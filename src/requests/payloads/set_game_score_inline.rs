use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::Message,
};

/// Use this method to set the score of the specified user in a game. On success, if the message was sent by the bot, returns the edited Message, otherwise returns True. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetGameScoreInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// User identifier
    user_id: i32,
    /// New score, must be non-negative
    score: i32,
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    disable_edit_message: Option<bool>,
}

impl Method for SetGameScoreInline {
    type Output = Message;

    const NAME: &'static str = "setGameScore";
}

impl json::Payload for SetGameScoreInline {}

impl dynamic::Payload for SetGameScoreInline {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SetGameScoreInline {
    pub fn new<I>(inline_message_id: I, user_id: i32, score: i32) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            user_id,
            score,
            force: None,
            disable_edit_message: None,
        }
    }
}

impl json::Request<'_, SetGameScoreInline> {
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.inline_message_id = val.into();
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
