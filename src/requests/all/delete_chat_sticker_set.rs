use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to delete a group sticker set from a supergroup.
///
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights. Use the field `can_set_sticker_set` optionally
/// returned in [`Bot::get_chat`] requests to check if the bot can use this
/// method.
///
/// [The official docs](https://core.telegram.org/bots/api#deletechatstickerset).
///
/// [`Bot::get_chat`]: crate::Bot::get_chat
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatStickerSet {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for DeleteChatStickerSet {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteChatStickerSet",
            &self,
        )
        .await
    }
}

impl DeleteChatStickerSet {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id }
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
}
