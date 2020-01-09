use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::WebhookInfo,
    Bot,
};

/// Use this method to get current webhook status. Requires no parameters. On
/// success, returns a WebhookInfo object. If the bot is using getUpdates, will
/// return an object with the url field empty.
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
