use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to set a new group sticker set for a supergroup.
///
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights. Use the field can_set_sticker_set optionally
/// returned in getChat requests to check if the bot can use this method.
///
/// [The official docs](https://core.telegram.org/bots/api#setchatstickerset).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatStickerSet {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    sticker_set_name: String,
}

#[async_trait::async_trait]
impl Request for SetChatStickerSet {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatStickerSet",
            &self,
        )
        .await
    }
}

impl SetChatStickerSet {
    pub(crate) fn new<C, S>(
        bot: Arc<Bot>,
        chat_id: C,
        sticker_set_name: S,
    ) -> Self
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        let chat_id = chat_id.into();
        let sticker_set_name = sticker_set_name.into();
        Self { bot, chat_id, sticker_set_name }
    }

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format `@supergroupusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Name of the sticker set to be set as the group sticker set.
    pub fn sticker_set_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker_set_name = val.into();
        self
    }
}
