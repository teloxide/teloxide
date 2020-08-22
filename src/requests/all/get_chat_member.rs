use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, ChatMember},
    Bot,
};

/// Use this method to get information about a member of a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#getchatmember).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetChatMember {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    user_id: i32,
}

#[async_trait::async_trait]
impl RequestOld for GetChatMember {
    type Output = ChatMember;

    async fn send(&self) -> ResponseResult<ChatMember> {
        net::request_json(self.bot.client(), self.bot.token(), "getChatMember", &self).await
    }
}

impl GetChatMember {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, user_id }
    }

    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
