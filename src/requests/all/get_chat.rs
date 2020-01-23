use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{Chat, ChatId},
    Bot,
};

/// Use this method to get up to date information about the chat (current name
/// of the user for one-on-one conversations, current username of a user, group
/// or channel, etc.).
///
/// [The official docs](https://core.telegram.org/bots/api#getchat).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct GetChat<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for GetChat<'_> {
    type Output = Chat;

    async fn send(&self) -> ResponseResult<Chat> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChat",
            &self,
        )
        .await
    }
}

impl<'a> GetChat<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
        }
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
