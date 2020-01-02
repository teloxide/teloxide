use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
};

/// Use this method to get information about a member of a chat. Returns a
/// ChatMember object on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
}

#[async_trait::async_trait]
impl Request<ChatMember> for GetChatMember {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<ChatMember> {
        network::request_json(
            bot.client(),
            bot.token(),
            "getChatMember",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl GetChatMember {
    pub fn new<C>(chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { chat_id, user_id }
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
