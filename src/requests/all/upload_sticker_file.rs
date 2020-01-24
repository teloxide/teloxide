use serde::Serialize;

use super::BotWrapper;
use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{File, InputFile},
    Bot,
};

/// Use this method to upload a .png file with a sticker for later use in
/// [`Bot::create_new_sticker_set`] and [`Bot::add_sticker_to_set`] methods (can
/// be used multiple times).
///
/// [The official docs](https://core.telegram.org/bots/api#uploadstickerfile).
///
/// [`Bot::create_new_sticker_set`]: crate::Bot::create_new_sticker_set
/// [`Bot::add_sticker_to_set`]: crate::Bot::add_sticker_to_set
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct UploadStickerFile<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    user_id: i32,
    png_sticker: InputFile,
}
#[async_trait::async_trait]
impl Request for UploadStickerFile<'_> {
    type Output = File;

    async fn send(&self) -> ResponseResult<File> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "uploadStickerFile",
            &self,
        )
        .await
    }
}

impl<'a> UploadStickerFile<'a> {
    pub(crate) fn new(
        bot: &'a Bot,
        user_id: i32,
        png_sticker: InputFile,
    ) -> Self {
        Self {
            bot: BotWrapper(bot),
            user_id,
            png_sticker,
        }
    }

    /// User identifier of sticker file owner.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// **Png** image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px. [More info on Sending Files »].
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn png_sticker(mut self, val: InputFile) -> Self {
        self.png_sticker = val;
        self
    }
}
