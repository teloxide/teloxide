use serde::{Deserialize, Serialize};
use reqwest::multipart::Form;

use crate::{
    requests::{dynamic, multipart, Method, form_builder::FormBuilder},
    types::{MaskPosition, InputFile, True},
};

/// Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    user_id: i32,
    /// Sticker set name
    name: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    png_sticker: InputFile,
    /// One or more emoji corresponding to the sticker
    emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    mask_position: Option<MaskPosition>,
}

impl Method for AddStickerToSet {
    type Output = True;

    const NAME: &'static str = "addStickerToSet";
}

impl multipart::Payload for AddStickerToSet {
    fn payload(&self) -> Form {
        FormBuilder::new()
            .add("user_id", &self.user_id)
            .add("name", &self.name)
            .add("png_sticker", &self.png_sticker)
            .add("emojis", &self.emojis)
            .add("mask_position", &self.mask_position)
            .build()
    }
}

impl dynamic::Payload for AddStickerToSet {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Multipart(multipart::Payload::payload(self))
    }
}

impl AddStickerToSet {
    pub fn new<N, P, E>(user_id: i32, name: N, png_sticker: P, emojis: E) -> Self
    where
        N: Into<String>,
        P: Into<InputFile>,
        E: Into<String>
    {
        let name = name.into();
        let png_sticker = png_sticker.into();
        let emojis = emojis.into();
        Self {
            user_id,
            name,
            png_sticker,
            emojis,
            mask_position: None,
        }
    }
}

impl multipart::Request<'_, AddStickerToSet> {
    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }

    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.name = val.into();
        self
    }

    pub fn png_sticker<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>
    {
        self.payload.png_sticker = val.into();
        self
    }

    pub fn emojis<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.emojis = val.into();
        self
    }

    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.payload.mask_position = Some(val);
        self
    }
}
