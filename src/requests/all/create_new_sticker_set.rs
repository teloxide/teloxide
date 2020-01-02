use serde::Serialize;

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{InputFile, MaskPosition, True},
    Bot,
};

/// Use this method to create new sticker set owned by a user. The bot will be
/// able to edit the created sticker set. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct CreateNewStickerSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// User identifier of created sticker set owner
    user_id: i32,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g.,
    /// animals). Can contain only english letters, digits and underscores.
    /// Must begin with a letter, can't contain consecutive underscores and
    /// must end in “_by_<bot username>”. <bot_username> is case insensitive.
    /// 1-64 characters.
    name: String,
    /// Sticker set title, 1-64 characters
    title: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px. Pass a file_id as a String to send a file that already
    /// exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. More info on Sending Files »
    png_sticker: InputFile,
    /// One or more emoji corresponding to the sticker
    emojis: String,
    /// Pass True, if a set of mask stickers should be created
    contains_masks: Option<bool>,
    /// A JSON-serialized object for position where the mask should be placed
    /// on faces
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl Request<True> for CreateNewStickerSet<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "createNewStickerSet",
            FormBuilder::new()
                .add("user_id", &self.user_id)
                .add("name", &self.name)
                .add("title", &self.title)
                .add("png_sticker", &self.png_sticker)
                .add("emojis", &self.emojis)
                .add("contains_masks", &self.contains_masks)
                .add("mask_position", &self.mask_position)
                .build(),
        )
        .await
    }
}

impl<'a> CreateNewStickerSet<'a> {
    pub(crate) fn new<N, T, E>(
        bot: &'a Bot,
        user_id: i32,
        name: N,
        title: T,
        png_sticker: InputFile,
        emojis: E,
    ) -> Self
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        let name = name.into();
        let title = title.into();
        let png_sticker = png_sticker.into();
        let emojis = emojis.into();
        Self {
            bot,
            user_id,
            name,
            title,
            png_sticker,
            emojis,
            contains_masks: None,
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

    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
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

    pub fn contains_masks(mut self, val: bool) -> Self {
        self.contains_masks = Some(val);
        self
    }

    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}
