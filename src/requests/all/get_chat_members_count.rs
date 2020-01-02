use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::ChatId,
    Bot,
};

/// Use this method to get the number of members in a chat. Returns Int on
/// success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetChatMembersCount<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request<i32> for GetChatMembersCount<'_> {
    async fn send(&self) -> ResponseResult<i32> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatMembersCount",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> GetChatMembersCount<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
}
