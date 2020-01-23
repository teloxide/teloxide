use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to delete a message, including service messages.
///
/// The limitations are:
///  - A message can only be deleted if it was sent less than 48 hours ago.
///  - Bots can delete outgoing messages in private chats, groups, and
///    supergroups.
///  - Bots can delete incoming messages in private chats.
///  - Bots granted can_post_messages permissions can delete outgoing messages
///    in channels.
///  - If the bot is an administrator of a group, it can delete any message
///    there.
///  - If the bot has can_delete_messages permission in a supergroup or a
///    channel, it can delete any message there.
///
/// [The official docs](https://core.telegram.org/bots/api#deletemessage).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct DeleteMessage<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
    message_id: i32,
}

#[async_trait::async_trait]
impl Request for DeleteMessage<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteMessage",
            &self,
        )
        .await
    }
}

impl<'a> DeleteMessage<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
            message_id,
        }
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

    /// Identifier of the message to delete.
    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }
}
