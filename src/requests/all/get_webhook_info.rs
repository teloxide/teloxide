use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::WebhookInfo,
    Bot,
};

/// Use this method to get current webhook status.
///
/// If the bot is using [`Bot::get_updates`], will return an object with the url
/// field empty.
///
/// [The official docs](https://core.telegram.org/bots/api#getwebhookinfo).
///
/// [`Bot::get_updates`]: crate::Bot::get_updates
#[derive(Debug, Clone, Serialize)]
pub struct GetWebhookInfo<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
}

#[async_trait::async_trait]
impl Request for GetWebhookInfo<'_> {
    type Output = WebhookInfo;

    async fn send(&self) -> ResponseResult<WebhookInfo> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getWebhookInfo",
            &self,
        )
        .await
    }
}

impl<'a> GetWebhookInfo<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }
}
