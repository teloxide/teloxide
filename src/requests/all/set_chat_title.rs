use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to change the title of a chat.
///
/// Titles can't be changed for private chats. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchattitle).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatTitle {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    title: String,
}

#[async_trait::async_trait]
impl Request for SetChatTitle {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatTitle",
            &self,
        )
        .await
    }
}

impl SetChatTitle {
    pub(crate) fn new<C, T>(bot: Arc<Bot>, chat_id: C, title: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        let chat_id = chat_id.into();
        let title = title.into();
        Self { bot, chat_id, title }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// New chat title, 1-255 characters.
    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
        self
    }
}
