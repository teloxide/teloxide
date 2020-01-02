use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, GameHighScore},
    Bot,
};

/// Use this method to get data for high score tables. Will return the score of
/// the specified user and several of his neighbors in a game. On success,
/// returns an Array of GameHighScore objects.This method will currently return
/// scores for the target user, plus two of his closest neighbors on each side.
/// Will also return the top three users if the user and his neighbors are not
/// among them. Please note that this behavior is subject to change.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetGameHighScores<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Target user id
    user_id: i32,
    /// Unique identifier for the target chat
    chat_id: ChatId,
    /// Identifier of the sent message
    message_id: i32,
}

#[async_trait::async_trait]
impl Request<Vec<GameHighScore>> for GetGameHighScores<'_> {
    async fn send(&self) -> ResponseResult<Vec<GameHighScore>> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getGameHighScores",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> GetGameHighScores<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        message_id: i32,
        user_id: i32,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            message_id,
            user_id,
        }
    }

    pub fn chat_id<C>(mut self, val: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
