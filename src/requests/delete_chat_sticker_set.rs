use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};
use async_trait::async_trait;

/// Use this method to delete a group sticker set from a supergroup. The bot
/// must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Use the field can_set_sticker_set optionally
/// returned in getChat requests to check if the bot can use this method.
/// Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatStickerSet<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    chat_id: ChatId,
}

#[async_trait]
impl Request for DeleteChatStickerSet<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl DeleteChatStickerSet<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteChatStickerSet",
            &self,
        )
        .await
    }
}

impl<'a> DeleteChatStickerSet<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }

    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }
}
