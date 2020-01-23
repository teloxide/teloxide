use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::True,
    Bot,
};

/// Use this method to move a sticker in a set created by the bot to a specific
/// position.
///
/// [The official docs](https://core.telegram.org/bots/api#setstickerpositioninset).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct SetStickerPositionInSet<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    sticker: String,
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
            &self,
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
            bot: BotWrapper(bot),
            sticker,
            position,
        }
    }

    /// File identifier of the sticker.
    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker = val.into();
        self
    }

    /// New sticker position in the set, zero-based.
    pub fn position(mut self, val: i32) -> Self {
        self.position = val;
        self
    }
}
