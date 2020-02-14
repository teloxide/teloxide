use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};
use std::sync::Arc;

/// Use this method to remove webhook integration if you decide to switch back
/// to [Bot::get_updates].
///
/// [The official docs](https://core.telegram.org/bots/api#deletewebhook).
///
/// [Bot::get_updates]: crate::Bot::get_updates
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteWebhook {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
}

#[async_trait::async_trait]
impl Request for DeleteWebhook {
    type Output = True;

    #[allow(clippy::trivially_copy_pass_by_ref)]
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

impl DeleteWebhook {
    pub(crate) fn new(bot: Arc<Bot>) -> Self {
        Self { bot }
    }
}
