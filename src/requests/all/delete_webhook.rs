use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};

/// Use this method to remove webhook integration if you decide to switch back
/// to [Bot::get_updates].
///
/// [The official docs](https://core.telegram.org/bots/api#deletewebhook).
///
/// [Bot::get_updates]: crate::Bot::get_updates
#[serde_with_macros::skip_serializing_none]
#[derive(Copy, Eq, PartialEq, Debug, Clone, Serialize)]
pub struct DeleteWebhook<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
}

#[async_trait::async_trait]
impl Request for DeleteWebhook<'_> {
    type Output = True;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
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
        Self {
            bot: BotWrapper(bot),
        }
    }
}
