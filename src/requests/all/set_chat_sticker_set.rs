use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to set a new group sticker set for a supergroup. The bot
/// must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Use the field can_set_sticker_set optionally
/// returned in getChat requests to check if the bot can use this method.
/// Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatStickerSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    chat_id: ChatId,
    /// Name of the sticker set to be set as the group sticker set
    sticker_set_name: String,
}

#[async_trait::async_trait]
impl Request<True> for SetChatStickerSet<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatStickerSet",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SetChatStickerSet<'a> {
    pub(crate) fn new<C, S>(
        bot: &'a Bot,
        chat_id: C,
        sticker_set_name: S,
    ) -> Self
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        let chat_id = chat_id.into();
        let sticker_set_name = sticker_set_name.into();
        Self {
            bot,
            chat_id,
            sticker_set_name,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn sticker_set_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.sticker_set_name = val.into();
        self
    }
}
