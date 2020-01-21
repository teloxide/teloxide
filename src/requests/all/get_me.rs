use crate::{
    network,
    requests::{Request, ResponseResult},
    types::User,
    Bot,
};
use serde::Serialize;

/// A simple method for testing your bot's auth token. Requires no parameters.
///
/// [The official docs](https://core.telegram.org/bots/api#getme).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct GetMe<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
}

#[async_trait::async_trait]
impl Request for GetMe<'_> {
    type Output = User;

    /// Returns basic information about the bot.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn send(&self) -> ResponseResult<User> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getMe",
            &self,
        )
        .await
    }
}

impl<'a> GetMe<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }
}
