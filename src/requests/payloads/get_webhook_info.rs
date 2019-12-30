use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::WebhookInfo,
};

/// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize, Default)]
pub struct GetWebhookInfo {}

impl Method for GetWebhookInfo {
    type Output = WebhookInfo;

    const NAME: &'static str = "getWebhookInfo";
}

impl json::Payload for GetWebhookInfo {}

impl dynamic::Payload for GetWebhookInfo {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetWebhookInfo {
    pub fn new() -> Self {
        Self {}
    }
}
