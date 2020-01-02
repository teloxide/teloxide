use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

/// Use this method to delete a group sticker set from a supergroup. The bot
/// must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Use the field can_set_sticker_set optionally
/// returned in getChat requests to check if the bot can use this method.
/// Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request<True> for DeleteChatStickerSet {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "deleteChatStickerSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl DeleteChatStickerSet {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { chat_id }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
}
