use crate::{
    net,
    requests::{Request, ResponseResult},
    types::User,
    Bot,
};
use serde::Serialize;

/// A filter method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
///
/// [`User`]: crate::types::User
#[derive(Debug, Clone, Copy, Serialize)]
pub struct GetMe<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
}

#[async_trait::async_trait]
impl Request for GetMe<'_> {
    type Output = User;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn send(&self) -> ResponseResult<User> {
        net::request_json(self.bot.client(), self.bot.token(), "getMe", &self)
            .await
    }
}

impl<'a> GetMe<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }
}
