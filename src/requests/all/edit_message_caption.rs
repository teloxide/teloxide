use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};
use std::sync::Arc;

/// Use this method to edit captions of messages.
///
/// On success, if edited message is sent by the bot, the edited [`Message`] is
/// returned, otherwise [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagecaption).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageCaption {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageCaption",
            &self,
        )
        .await
    }
}

impl EditMessageCaption {
    pub(crate) fn new(
        bot: Arc<Bot>,
        chat_or_inline_message: ChatOrInlineMessage,
    ) -> Self {
        Self {
            bot,
            chat_or_inline_message,
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// New caption of the message.
    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
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
