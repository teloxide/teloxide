use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{GameHighScore, TargetMessage},
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
/// [The official docs](https://core.telegram.org/bots/api#getgamehighscores)
#[derive(Debug, Clone, Serialize)]
pub struct GetGameHighScores {
    #[serde(skip_serializing)]
    bot: Bot,
    #[serde(flatten)]
    target: TargetMessage,
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
    pub(crate) fn new<T>(bot: Bot, target: T, user_id: i32) -> Self
    where
        T: Into<TargetMessage>,
    {
        let target = target.into();
        Self { bot, target, user_id }
    }

    /// Target message, either chat id and message id or inline message id.
    pub fn target<T>(mut self, val: T) -> Self
    where
        T: Into<TargetMessage>,
    {
        self.target = val.into();
        self
    }

    /// Target user id.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
