use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{MaskPosition, StickerType, True},
    Bot,
};

/// Use this method to create new sticker set owned by a user. The bot will be
/// able to edit the created sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#createnewstickerset).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct CreateNewStickerSet {
    #[serde(skip_serializing)]
    bot: Bot,
    user_id: i32,
    name: String,
    title: String,
    #[serde(flatten)]
    sticker_type: StickerType,
    emojis: String,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl RequestOld for CreateNewStickerSet {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_multipart(self.bot.client(), self.bot.token(), "createNewStickerSet", self)
            .await
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
