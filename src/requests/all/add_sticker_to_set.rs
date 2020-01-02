use serde::Serialize;

use crate::{
    network,
    requests::form_builder::FormBuilder,
    types::{InputFile, MaskPosition, True},
    Bot,
};

use crate::requests::{Request, ResponseResult};

/// Use this method to add a new sticker to a set created by the bot. Returns
/// True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct AddStickerToSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// User identifier of sticker set owner
    user_id: i32,
    /// Sticker set name
    name: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px. Pass a file_id as a String to send a file that already
    /// exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. More info on Sending Files »
    png_sticker: InputFile,
    /// One or more emoji corresponding to the sticker
    emojis: String,
    /// A JSON-serialized object for position where the mask should be placed
    /// on faces
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl Request<True> for AddStickerToSet<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "addStickerToSet",
            FormBuilder::new()
                .add("user_id", &self.user_id)
                .add("name", &self.name)
                .add("png_sticker", &self.png_sticker)
                .add("emojis", &self.emojis)
                .add("mask_position", &self.mask_position)
                .build(),
        )
        .await
    }
}

impl<'a> AddStickerToSet<'a> {
    pub(crate) fn new<N, E>(
        bot: &'a Bot,
        user_id: i32,
        name: N,
        png_sticker: InputFile,
        emojis: E,
    ) -> Self
    where
        N: Into<String>,
        E: Into<String>,
    {
        let name = name.into();
        let png_sticker = png_sticker.into();
        let emojis = emojis.into();
        Self {
            bot,
            user_id,
            name,
            png_sticker,
            emojis,
            mask_position: None,
        }
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }

    pub fn png_sticker(mut self, val: InputFile) -> Self {
        self.png_sticker = val.into();
        self
    }

    pub fn emojis<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.emojis = val.into();
        self
    }

    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}
