use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::StickerSet,
};

/// Use this method to get a sticker set. On success, a StickerSet object is
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    name: String,
}

#[async_trait::async_trait]
impl Request<StickerSet> for GetStickerSet {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<StickerSet> {
        network::request_json(
            bot.client(),
            bot.token(),
            "getStickerSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl GetStickerSet {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        let name = name.into();
        Self { name }
    }

    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }
}
