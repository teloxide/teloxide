use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::User,
};

#[derive(Debug, Clone)]
/// A filter method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a [`User`] object.
pub struct GetMe<'a> {
    bot: &'a Bot,
}

#[async_trait]
impl Request for GetMe<'_> {
    type Output = User;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl GetMe<'_> {
    pub async fn send(self) -> ResponseResult<User> {
        network::request_simple(self.bot.client(), self.bot.token(), "getMe")
            .await
    }
}

impl<'a> GetMe<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        GetMe { bot }
    }
}
