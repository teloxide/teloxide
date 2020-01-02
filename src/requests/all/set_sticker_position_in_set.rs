use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
};

/// Use this method to move a sticker in a set created by the bot to a specific
/// position . Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    sticker: String,
    /// New sticker position in the set, zero-based
    position: i32,
}

#[async_trait::async_trait]
impl Request<True> for SetStickerPositionInSet {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "setStickerPositionInSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl SetStickerPositionInSet {
    pub fn new<S>(sticker: S, position: i32) -> Self
    where
        S: Into<String>,
    {
        let sticker = sticker.into();
        Self { sticker, position }
    }

    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker = val.into();
        self
    }

    pub fn position(mut self, val: i32) -> Self {
        self.position = val;
        self
    }
}
