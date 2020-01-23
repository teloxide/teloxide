use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
    Bot,
};

/// Use this method to get a list of administrators in a chat.
///
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
///
/// [The official docs](https://core.telegram.org/bots/api#getchatadministrators).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct GetChatAdministrators<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for GetChatAdministrators<'_> {
    type Output = Vec<ChatMember>;

    /// On success, returns an array that contains information about all chat
    /// administrators except other bots.
    async fn send(&self) -> ResponseResult<Vec<ChatMember>> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatAdministrators",
            &self,
        )
        .await
    }
}

impl<'a> GetChatAdministrators<'a> {
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
