use crate::{
    net,
    requests::{Request, ResponseResult},
    types::Me,
    Bot,
};
use serde::Serialize;
use std::sync::Arc;

/// A simple method for testing your bot's auth token. Requires no parameters.
///
/// [The official docs](https://core.telegram.org/bots/api#getme).
#[derive(Debug, Clone, Serialize)]
pub struct GetMe {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
}

#[async_trait::async_trait]
impl Request for GetMe {
    type Output = Me;

    /// Returns basic information about the bot.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn send(&self) -> ResponseResult<Me> {
        net::request_json(self.bot.client(), self.bot.token(), "getMe", &self)
            .await
    }
}

impl GetMe {
    pub(crate) fn new(bot: Arc<Bot>) -> Self {
        Self { bot }
    }
}
