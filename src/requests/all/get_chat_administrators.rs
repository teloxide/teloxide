use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
    Bot,
};

/// Use this method to get a list of administrators in a chat. On success,
/// returns an Array of ChatMember objects that contains information about all
/// chat administrators except other bots. If the chat is a group or a
/// supergroup and no administrators were appointed, only the creator will be
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetChatAdministrators<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for GetChatAdministrators<'_> {
    type Output = Vec<ChatMember>;

    async fn send(&self) -> ResponseResult<Vec<ChatMember>> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatAdministrators",
            &serde_json::to_string(self).unwrap(),
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
