use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};

/// Use this method to remove webhook integration if you decide to switch back
/// to getUpdates. Returns True on success. Requires no parameters.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteWebhook<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
}

#[async_trait::async_trait]
impl Request<True> for DeleteWebhook<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteWebhook",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }
}
