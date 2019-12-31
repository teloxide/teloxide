use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::True,
};

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    sticker: String,
    /// New sticker position in the set, zero-based
    position: i32,
}

impl Method for SetStickerPositionInSet {
    type Output = True;

    const NAME: &'static str = "setStickerPositionInSet";
}

impl json::Payload for SetStickerPositionInSet {}

impl dynamic::Payload for SetStickerPositionInSet {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SetStickerPositionInSet {
    pub fn new<S>(sticker: S, position: i32) -> Self
    where
        S: Into<String>
    {
        let sticker = sticker.into();
        Self {
            sticker,
            position,
        }
    }
}

impl json::Request<'_, SetStickerPositionInSet> {
    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.sticker = val.into();
        self
    }

    pub fn position(mut self, val: i32) -> Self {
        self.payload.position = val;
        self
    }
}
                 