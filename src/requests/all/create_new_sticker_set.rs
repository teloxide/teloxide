use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{InputFile, MaskPosition, True},
    Bot,
};

/// Use this method to create new sticker set owned by a user. The bot will be
/// able to edit the created sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#createnewstickerset).
#[derive(Debug, Clone)]
pub struct CreateNewStickerSet<'a> {
    bot: &'a Bot,
    user_id: i32,
    name: String,
    title: String,
    png_sticker: InputFile,
    emojis: String,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl Request for CreateNewStickerSet<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "createNewStickerSet",
            FormBuilder::new()
                .add("user_id", &self.user_id)
                .await
                .add("name", &self.name)
                .await
                .add("title", &self.title)
                .await
                .add("png_sticker", &self.png_sticker)
                .await
                .add("emojis", &self.emojis)
                .await
                .add("contains_masks", &self.contains_masks)
                .await
                .add("mask_position", &self.mask_position)
                .await
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
        Self {
            bot,
            user_id,
            name: name.into(),
            title: title.into(),
            png_sticker,
            emojis: emojis.into(),
            contains_masks: None,
            mask_position: None,
        }
    }

    /// User identifier of created sticker set owner.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g.,
    /// animals). Can contain only english letters, digits and underscores. Must
    /// begin with a letter, can't contain consecutive underscores and must end
    /// in `_by_<bot username>`. `<bot_username>` is case insensitive. 1-64
    /// characters.
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }

    /// Sticker set title, 1-64 characters.
    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
        self
    }

    /// **Png** image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px. Pass a file_id as a String to send a file that already
    /// exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. [More info on Sending Files »].
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn png_sticker(mut self, val: InputFile) -> Self {
        self.png_sticker = val;
        self
    }

    /// One or more emoji corresponding to the sticker.
    pub fn emojis<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.emojis = val.into();
        self
    }

    /// Pass `true`, if a set of mask stickers should be created.
    pub fn contains_masks(mut self, val: bool) -> Self {
        self.contains_masks = Some(val);
        self
    }

    /// A JSON-serialized object for position where the mask should be placed on
    /// faces.
    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}
