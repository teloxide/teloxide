use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
    Bot,
};

/// Use this method to get information about a member of a chat. Returns a
/// ChatMember object on success.
#[serde_with_macros::skip_serializing_none]
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

#[async_trait::async_trait]
impl Request for GetChatMember<'_> {
    type Output = ChatMember;

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
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            user_id,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
