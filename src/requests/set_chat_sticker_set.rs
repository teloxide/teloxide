use crate::bot::Bot;
use crate::network;
use crate::requests::{Request, ResponseResult};
use crate::types::{ChatId, True};
use async_trait::async_trait;

/// Use this method to set a new group sticker set for a supergroup. The bot
/// must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Use the field can_set_sticker_set optionally
/// returned in getChat requests to check if the bot can use this method.
/// Returns True on success.
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

#[async_trait]
impl Request for SetChatStickerSet<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SetChatStickerSet<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatStickerSet",
            &self,
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
        Self {
            bot,
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }

    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn sticker_set_name<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.sticker_set_name = value.into();
        self
    }
}
