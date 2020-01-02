use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::WebhookInfo,
};

/// Use this method to get current webhook status. Requires no parameters. On
/// success, returns a WebhookInfo object. If the bot is using getUpdates, will
/// return an object with the url field empty.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize, Default)]
pub struct GetWebhookInfo;

#[async_trait::async_trait]
impl Request<WebhookInfo> for GetWebhookInfo {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<WebhookInfo> {
        network::request_json(
            bot.client(),
            bot.token(),
            "getWebhookInfo",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl GetWebhookInfo {
    pub fn new() -> Self {
        Self
    }
}
