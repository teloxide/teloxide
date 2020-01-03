use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};

/// Use this method to delete a sticker from a set created by the bot. Returns
/// True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStickerFromSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// File identifier of the sticker
    sticker: String,
}

#[async_trait::async_trait]
impl Request for DeleteStickerFromSet<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteStickerFromSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> DeleteStickerFromSet<'a> {
    pub(crate) fn new<S>(bot: &'a Bot, sticker: S) -> Self
    where
        S: Into<String>,
    {
        let sticker = sticker.into();
        Self { bot, sticker }
    }

    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker = val.into();
        self
    }
}
