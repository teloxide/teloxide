use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to change the description of a group, a supergroup or a
/// channel.
///
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchatdescription).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatDescription {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    description: Option<String>,
}

#[async_trait::async_trait]
impl Request for SetChatDescription {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatDescription",
            &self,
        )
        .await
    }
}

impl SetChatDescription {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, description: None }
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

    /// New chat description, 0-255 characters.
    pub fn description<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(val.into());
        self
    }
}
