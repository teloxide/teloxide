use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to unpin a message in a group, a supergroup, or a channel.
/// The bot must be an administrator in the chat for this to work and must have
/// the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’
/// admin right in the channel. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessage<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for UnpinChatMessage<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "unpinChatMessage",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> UnpinChatMessage<'a> {
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
