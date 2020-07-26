use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, GameHighScore},
    Bot,
};

/// Use this method to get data for high score tables.
///
/// Will return the score of the specified user and several of his neighbors in
/// a game.
///
/// ## Note
/// This method will currently return scores for the target user, plus two of
/// his closest neighbors on each side. Will also return the top three users if
/// the user and his neighbors are not among them. Please note that this
/// behavior is subject to change.
///
/// [The official docs](https://core.telegram.org/bots/api#getgamehighscores).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetGameHighScores {
    #[serde(skip_serializing)]
    bot: Bot,
    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,
    user_id: i32,
}

#[async_trait::async_trait]
impl Request for GetGameHighScores {
    type Output = Vec<GameHighScore>;

    async fn send(&self) -> ResponseResult<Vec<GameHighScore>> {
        net::request_json(self.bot.client(), self.bot.token(), "getGameHighScores", &self).await
    }
}

impl GetGameHighScores {
    pub(crate) fn new(bot: Bot, chat_or_inline_message: ChatOrInlineMessage, user_id: i32) -> Self {
        Self { bot, chat_or_inline_message, user_id }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// Target user id.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
