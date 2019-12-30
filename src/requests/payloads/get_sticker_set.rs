use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::StickerSet,
};

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    name: String,
}

impl Method for GetStickerSet {
    type Output = StickerSet;

    const NAME: &'static str = "getStickerSet";
}

impl json::Payload for GetStickerSet {}

impl dynamic::Payload for GetStickerSet {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetStickerSet {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>
    {
        let name = name.into();
        Self {
            name,
        }
    }
}

impl json::Request<'_, GetStickerSet> {
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.name = val.into();
        self
    }
}
                 