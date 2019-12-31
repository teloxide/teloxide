use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::GameHighScore,
};
use crate::types::ChatId;

/// Use this method to get data for high score tables. Will return the score of the specified user and several of his neighbors in a game. On success, returns an Array of GameHighScore objects.This method will currently return scores for the target user, plus two of his closest neighbors on each side. Will also return the top three users if the user and his neighbors are not among them. Please note that this behavior is subject to change.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetGameHighScore {
    /// Target user id
    user_id: i32,
    /// Unique identifier for the target chat
    chat_id: ChatId,
    /// Identifier of the sent message
    message_id: i32,
}

impl Method for GetGameHighScore {
    type Output = Vec<GameHighScore>;

    const NAME: &'static str = "getGameHighScores";
}

impl json::Payload for GetGameHighScore {}

impl dynamic::Payload for GetGameHighScore {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetGameHighScore {
    pub fn new<C>(chat_id: C, message_id: i32, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            message_id,
            user_id,
        }
    }
}

impl json::Request<'_, GetGameHighScore> {
    pub fn chat_id<C>(mut self, val: C) -> Self
    where
        C: Into<ChatId>,
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
}
                 