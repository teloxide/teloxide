use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::BotCommand,
    Bot,
};

/// Use this method to get the current list of the bot's commands.
///
/// [The official docs](https://core.telegram.org/bots/api#getmycommands).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetMyCommands {
    #[serde(skip_serializing)]
    bot: Bot,
}

#[async_trait::async_trait]
impl Request for GetMyCommands {
    type Output = Vec<BotCommand>;

    async fn send(&self) -> ResponseResult<Self::Output> {
        net::request_json(self.bot.client(), self.bot.token(), "getMyCommands", &self).await
    }
}

impl GetMyCommands {
    pub(crate) fn new(bot: Bot) -> Self {
        Self { bot }
    }
}
