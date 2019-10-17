use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{Chat, ChatId},
};

/// Use this method to get the number of members in a chat. Returns Int on
/// success.
#[derive(Debug, Clone, Serialize)]
pub struct GetChatMembersCount<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username
    /// of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait]
impl Request for GetChatMembersCount<'_> {
    type Output = Chat;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl GetChatMembersCount<'_> {
    pub async fn send(self) -> ResponseResult<Chat> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatMembersCount",
            &self,
        )
        .await
    }
}

impl<'a> GetChatMembersCount<'a> {
    pub fn new<C>(bot: &'a Bot, chat_id: C) -> Self
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
