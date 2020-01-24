use serde::Serialize;

use super::BotWrapper;
use crate::{
    net,
    requests::{Request, ResponseResult},
    types::StickerSet,
    Bot,
};

/// Use this method to get a sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#getstickerset).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct GetStickerSet<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    name: String,
}

#[async_trait::async_trait]
impl Request for GetStickerSet<'_> {
    type Output = StickerSet;

    async fn send(&self) -> ResponseResult<StickerSet> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getStickerSet",
            &self,
        )
        .await
    }
}

impl<'a> GetStickerSet<'a> {
    pub(crate) fn new<N>(bot: &'a Bot, name: N) -> Self
    where
        N: Into<String>,
    {
        let name = name.into();
        Self {
            bot: BotWrapper(bot),
            name,
        }
    }

    /// Name of the sticker set.
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }
}
