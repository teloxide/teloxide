use serde::Serialize;

use crate::{
    net,
    types::{MaskPosition, True},
    Bot,
};

use crate::{
    requests::{Request, ResponseResult},
    types::StickerType,
};

/// Use this method to add a new sticker to a set created by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#addstickertoset).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct AddStickerToSet {
    #[serde(skip_serializing)]
    bot: Bot,
    pub user_id: i32,
    pub name: String,
    #[serde(flatten)]
    pub sticker_type: StickerType,
    pub emojis: String,
    pub mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl Request for AddStickerToSet {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_multipart(self.bot.client(), self.bot.token(), "addStickerToSet", self).await
    }
}

impl AddStickerToSet {
    pub(crate) fn new<N, E>(
        bot: Bot,
        user_id: i32,
        name: N,
        sticker_type: StickerType,
        emojis: E,
    ) -> Self
    where
        N: Into<String>,
        E: Into<String>,
    {
        Self {
            bot,
            user_id,
            name: name.into(),
            sticker_type,
            emojis: emojis.into(),
            mask_position: None,
        }
    }

    /// User identifier of sticker set owner.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// Sticker set name.
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
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

    /// A JSON-serialized object for position where the mask should be placed on
    /// faces.
    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}
