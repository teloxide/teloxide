use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::ChatId,
};

/// Use this method to get the number of members in a chat. Returns Int on
/// success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request<i32> for GetChatMembersCount {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<i32> {
        network::request_json(
            bot.client(),
            bot.token(),
            "getChatMembersCount",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl GetChatMembersCount {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { chat_id }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
}
