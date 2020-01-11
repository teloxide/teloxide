use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::StickerSet,
    Bot,
};

/// Use this method to get a sticker set. On success, a StickerSet object is
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetStickerSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Name of the sticker set
    name: String,
}

#[async_trait::async_trait]
impl Request for GetStickerSet<'_> {
    type Output = StickerSet;

    async fn send(&self) -> ResponseResult<StickerSet> {
        network::request_json(
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
        Self { bot, name }
    }

    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }
}
