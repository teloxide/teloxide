use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
};

/// Use this method to delete a sticker from a set created by the bot. Returns
/// True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    sticker: String,
}

#[async_trait::async_trait]
impl Request<True> for DeleteStickerFromSet {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "deleteStickerFromSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl DeleteStickerFromSet {
    pub fn new<S>(sticker: S) -> Self
    where
        S: Into<String>,
    {
        let sticker = sticker.into();
        Self { sticker }
    }

    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker = val.into();
        self
    }
}
