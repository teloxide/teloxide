use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::BotCommands,
    Bot,
};
use std::sync::Arc;

/// Use this method to get the current list of the bot's commands.
/// Requires no parameters.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetMyCommands {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
}

#[async_trait::async_trait]
impl Request for GetMyCommands {
    type Output = Vec<BotCommands>;

    /// Returns Array of BotCommand on success.
    async fn send(&self) -> ResponseResult<Vec<BotCommands>> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getMyCommands",
            &self,
        )
        .await
    }
}

impl GetMyCommands {
    pub(crate) fn new(bot: Arc<Bot>) -> Self {
        Self { bot }
    }
}
