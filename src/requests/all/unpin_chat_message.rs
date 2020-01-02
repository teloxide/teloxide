use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

/// Use this method to unpin a message in a group, a supergroup, or a channel.
/// The bot must be an administrator in the chat for this to work and must have
/// the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’
/// admin right in the channel. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request<True> for UnpinChatMessage {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "unpinChatMessage",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl UnpinChatMessage {
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
