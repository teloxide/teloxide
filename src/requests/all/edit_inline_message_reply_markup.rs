use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, True},
    Bot,
};

/// Use this method to edit only the reply markup of messages sent via the bot.
///
/// On success, [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagereplymarkup).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditInlineMessageReplyMarkup {
    #[serde(skip_serializing)]
    bot: Bot,
    inline_message_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditInlineMessageReplyMarkup {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(self.bot.client(), self.bot.token(), "editMessageReplyMarkup", &self)
            .await
    }
}

impl EditInlineMessageReplyMarkup {
    pub(crate) fn new<I>(bot: Bot, inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self { bot, inline_message_id, reply_markup: None }
    }

    /// Identifier of the inline message.
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
        self
    }

    /// A JSON-serialized object for an [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
