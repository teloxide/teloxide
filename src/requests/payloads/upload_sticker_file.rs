use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{InputFile, File},
};

/// Use this method to upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    user_id: i32,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. More info on Sending Files »
    png_sticker: InputFile,
}

impl Method for UploadStickerFile {
    type Output = File;

    const NAME: &'static str = "uploadStickerFile";
}

impl json::Payload for UploadStickerFile {}

impl dynamic::Payload for UploadStickerFile {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl UploadStickerFile {
    pub fn new(user_id: i32, png_sticker: InputFile) -> Self {
        Self {
            user_id,
            png_sticker,
        }
    }
}

impl json::Request<'_, UploadStickerFile> {
    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }

    pub fn png_sticker(mut self, val: InputFile) -> Self {
        self.payload.png_sticker = val;
        self
    }
}
                 