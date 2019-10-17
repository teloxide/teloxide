use crate::bot::Bot;
use crate::network;
use crate::requests::{Request, ResponseResult};
use crate::types::{ChatId, ChatMember};
use async_trait::async_trait;

/// Use this method to get information about a member of a chat. Returns a
/// ChatMember object on success.
#[derive(Debug, Clone, Serialize)]
pub struct GetChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,

    /// Unique identifier of the target user
    user_id: i32,
}

#[async_trait]
impl Request for GetChatMember<'_> {
    type Output = ChatMember;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl GetChatMember<'_> {
    async fn send(&self) -> ResponseResult<ChatMember> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatMember",
            &self,
        )
        .await
    }
}

impl<'a> GetChatMember<'a> {
    pub(crate) fn new<C, I>(bot: &'a Bot, chat_id: C, user_id: I) -> Self
    where
        C: Into<ChatId>,
        I: Into<i32>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id: user_id.into(),
        }
    }

    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn user_id<I>(mut self, value: I) -> Self
    where
        I: Into<i32>,
    {
        self.user_id = value.into();
        self
    }
}
