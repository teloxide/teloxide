use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::True,
};

/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success. Requires no parameters.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize, Default)]
pub struct DeleteWebhook {}

impl Method for DeleteWebhook {
    type Output = True;

    const NAME: &'static str = "deleteWebhook";
}

impl json::Payload for DeleteWebhook {}

impl dynamic::Payload for DeleteWebhook {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl DeleteWebhook {
    pub fn new() -> Self {
        Self {}
    }
}
                 