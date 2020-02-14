use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
    Bot,
};
use std::sync::Arc;

/// Use this method to get a list of administrators in a chat.
///
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
///
/// [The official docs](https://core.telegram.org/bots/api#getchatadministrators).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetChatAdministrators {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for GetChatAdministrators {
    type Output = Vec<ChatMember>;

    /// On success, returns an array that contains information about all chat
    /// administrators except other bots.
    async fn send(&self) -> ResponseResult<Vec<ChatMember>> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatAdministrators",
            &self,
        )
        .await
    }
}

impl GetChatAdministrators {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id }
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
}
