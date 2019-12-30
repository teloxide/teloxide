use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::True,
};

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    sticker: String,
}

impl Method for DeleteStickerFromSet {
    type Output = True;

    const NAME: &'static str = "deleteStickerFromSet";
}

impl json::Payload for DeleteStickerFromSet {}

impl dynamic::Payload for DeleteStickerFromSet {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl DeleteStickerFromSet {
    pub fn new<S>(sticker: S) -> Self
    where
        S: Into<String>
    {
        let sticker = sticker.into();
        Self {
            sticker,
        }
    }
}

impl json::Request<'_, DeleteStickerFromSet> {
    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.sticker = val.into();
        self
    }
}
                 