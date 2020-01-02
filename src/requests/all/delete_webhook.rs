use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
};

/// Use this method to remove webhook integration if you decide to switch back
/// to getUpdates. Returns True on success. Requires no parameters.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize, Default)]
pub struct DeleteWebhook;

#[async_trait::async_trait]
impl Request<True> for DeleteWebhook {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "deleteWebhook",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl DeleteWebhook {
    pub fn new() -> Self {
        Self
    }
}
