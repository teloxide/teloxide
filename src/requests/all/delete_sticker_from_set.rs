use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};
use std::sync::Arc;

/// Use this method to delete a sticker from a set created by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#deletestickerfromset).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStickerFromSet {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    sticker: String,
}

#[async_trait::async_trait]
impl Request for DeleteStickerFromSet {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteStickerFromSet",
            &self,
        )
        .await
    }
}

impl DeleteStickerFromSet {
    pub(crate) fn new<S>(bot: Arc<Bot>, sticker: S) -> Self
    where
        S: Into<String>,
    {
        let sticker = sticker.into();
        Self { bot, sticker }
    }

    /// File identifier of the sticker.
    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker = val.into();
        self
    }
}
