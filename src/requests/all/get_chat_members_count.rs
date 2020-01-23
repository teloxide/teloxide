use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::ChatId,
    Bot,
};

/// Use this method to get the number of members in a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#getchatmemberscount).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct GetChatMembersCount<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for GetChatMembersCount<'_> {
    type Output = i32;

    async fn send(&self) -> ResponseResult<i32> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatMembersCount",
            &self,
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
