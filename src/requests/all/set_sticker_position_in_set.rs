use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};

/// Use this method to move a sticker in a set created by the bot to a specific
/// position . Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerPositionInSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// File identifier of the sticker
    sticker: String,
    /// New sticker position in the set, zero-based
    position: i32,
}

#[async_trait::async_trait]
impl Request for SetStickerPositionInSet<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setStickerPositionInSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SetStickerPositionInSet<'a> {
    pub(crate) fn new<S>(bot: &'a Bot, sticker: S, position: i32) -> Self
    where
        S: Into<String>,
    {
        let sticker = sticker.into();
        Self {
            bot,
            sticker,
            position,
        }
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
