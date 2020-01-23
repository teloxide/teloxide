use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, Poll},
    Bot,
};

/// Use this method to stop a poll which was sent by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#stoppoll).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct StopPoll<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
    message_id: i32,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for StopPoll<'_> {
    type Output = Poll;

    /// On success, the stopped [`Poll`] with the final results is returned.
    ///
    /// [`Poll`]: crate::types::Poll
    async fn send(&self) -> ResponseResult<Poll> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "stopPoll",
            &self,
        )
        .await
    }
}
impl<'a> StopPoll<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
            message_id,
            reply_markup: None,
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

    /// Identifier of the original message with the poll.
    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
