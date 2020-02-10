use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::WebhookInfo,
    Bot,
};
use std::sync::Arc;

/// Use this method to get current webhook status.
///
/// If the bot is using [`Bot::get_updates`], will return an object with the url
/// field empty.
///
/// [The official docs](https://core.telegram.org/bots/api#getwebhookinfo).
///
/// [`Bot::get_updates`]: crate::Bot::get_updates
#[derive(Debug, Clone, Serialize)]
pub struct GetWebhookInfo {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
}

#[async_trait::async_trait]
impl Request for GetWebhookInfo {
    type Output = WebhookInfo;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn send(&self) -> ResponseResult<WebhookInfo> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getWebhookInfo",
            &self,
        )
        .await
    }
}

impl GetWebhookInfo {
    pub(crate) fn new(bot: Arc<Bot>) -> Self {
        Self { bot }
    }
}
