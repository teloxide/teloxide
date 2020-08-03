use crate::{
    net,
    requests::{form_builder::FormBuilder, RequestWithFile, ResponseResult},
    types::{MaskPosition, StickerType, True},
    Bot,
};

/// Use this method to create new sticker set owned by a user. The bot will be
/// able to edit the created sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#createnewstickerset).
#[derive(Debug, Clone)]
pub struct CreateNewStickerSet {
    bot: Bot,
    user_id: i32,
    name: String,
    title: String,
    sticker_type: StickerType,
    emojis: String,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl RequestWithFile for CreateNewStickerSet {
    type Output = True;

    async fn send(self) -> tokio::io::Result<ResponseResult<True>> {
        let builder = FormBuilder::new()
            .add_text("user_id", &self.user_id)
            .add_text("name", &self.name)
            .add_text("title", &self.title);

        let builder = match &self.sticker_type {
            StickerType::Png(file) => builder.add_input_file("png_sticker", &file),
            StickerType::Tgs(file) => builder.add_input_file("tgs_sticker", &file),
        }
        .await?
        .add_text("emojis", &self.emojis)
        .add_text("contains_masks", &self.contains_masks)
        .add_text("mask_position", &self.mask_position);

        Ok(net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "createNewStickerSet",
            builder.build(),
        )
        .await)
    }
}

impl CreateNewStickerSet {
    pub(crate) fn new<N, T, E>(
        bot: Bot,
        user_id: i32,
        name: N,
        title: T,
        sticker_type: StickerType,
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
            sticker_type,
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
    /// animals). Can contain only english letters, digits and underscores.
    ///
    /// Must begin with a letter, can't contain consecutive underscores and must
    /// end in `_by_<bot username>`. `<bot_username>` is case insensitive.
    /// 1-64 characters.
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

    pub fn sticker_type(mut self, val: StickerType) -> Self {
        self.sticker_type = val;
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
