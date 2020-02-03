use serde::Serialize;

use super::BotWrapper;
use crate::{
    net,
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
#[derive(Copy, Eq, PartialEq, Debug, Clone, Serialize)]
pub struct GetWebhookInfo<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
}

#[async_trait::async_trait]
impl Request for GetWebhookInfo<'_> {
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

impl<'a> GetWebhookInfo<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self {
            bot: BotWrapper(bot),
        }
    }
}
