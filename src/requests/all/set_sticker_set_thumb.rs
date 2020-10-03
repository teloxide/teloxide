use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InputFile, True},
    Bot,
};

/// Use this method to set the thumbnail of a sticker set. Animated thumbnails
/// can be set for animated sticker sets only.
///
/// [The official docs](https://core.telegram.org/bots/api#setstickersetthumb).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetThumb {
    #[serde(skip_serializing)]
    bot: Bot,
    name: String,
    user_id: i32,
    thumb: Option<InputFile>,
}

#[async_trait::async_trait]
impl Request for SetStickerSetThumb {
    type Output = True;

    async fn send(&self) -> ResponseResult<Self::Output> {
        net::request_json(self.bot.client(), self.bot.token(), "setStickerSetThumb", &self).await
    }
}

impl SetStickerSetThumb {
    pub(crate) fn new<S>(bot: Bot, name: S, user_id: i32) -> Self
    where
        S: Into<String>,
    {
        Self { bot, name: name.into(), user_id, thumb: None }
    }

    /// Sticker set name.
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }

    /// User identifier of the sticker set owner.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and
    /// have width and height exactly 100px, or a TGS animation with the
    /// thumbnail up to 32 kilobytes in size; see https://core.telegram.org/animated_stickers#technical-requirements
    /// for animated sticker technical requirements.
    ///
    /// Pass [`InputFile::FileId`] to send a file that exists on the Telegram
    /// servers (recommended), pass an [`InputFile::Url`] for Telegram to get a
    /// file from the Internet (20MB max.), pass [`InputFile::File`] to upload
    /// a file from the file system or [`InputFile::Memory`] to upload a file
    /// from memory (50MB max. each). [More info on Sending Files »]. Animated
    /// sticker set thumbnail can't be uploaded via HTTP URL.
    ///
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [`InputFile::Url]: crate::types::InputFile::Url
    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }
}
