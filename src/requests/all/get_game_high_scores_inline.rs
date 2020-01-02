use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::GameHighScore,
};

/// Use this method to get data for high score tables. Will return the score of
/// the specified user and several of his neighbors in a game. On success,
/// returns an Array of GameHighScore objects.This method will currently return
/// scores for the target user, plus two of his closest neighbors on each side.
/// Will also return the top three users if the user and his neighbors are not
/// among them. Please note that this behavior is subject to change.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetGameHighScoresInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// Target user id
    user_id: i32,
}

#[async_trait::async_trait]
impl Request<Vec<GameHighScore>> for GetGameHighScoresInline {
    async fn send(
        &self,
        bot: &crate::Bot,
    ) -> ResponseResult<Vec<GameHighScore>> {
        network::request_json(
            bot.client(),
            bot.token(),
            "getGameHighScores",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl GetGameHighScoresInline {
    pub fn new<I>(inline_message_id: I, user_id: i32) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            user_id,
        }
    }

    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
