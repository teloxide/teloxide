use serde::Serialize;

use crate::{
    net,
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
impl Request for DeleteWebhook<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteWebhook",
            &self,
        )
        .await
    }
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }
}
