use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
};
use async_trait::async_trait;

/// Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned
#[derive(Debug, Clone, Serialize)]
pub struct GetChatAdministrators<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait]
impl Request for GetChatAdministrators<'_> {
    type Output = Vec<ChatMember>;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl GetChatAdministrators<'_> {
    async fn send(&self) -> ResponseResult<Vec<ChatMember>> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatAdministrators",
            &self,
        )
        .await
    }
}

impl<'a> GetChatAdministrators<'a> {
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
