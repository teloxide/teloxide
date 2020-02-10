use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};
use std::sync::Arc;

/// Use this method to edit text and game messages.
///
/// On success, if edited message is sent by the bot, the edited [`Message`] is
/// returned, otherwise [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagetext).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageText {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,
    text: String,
    parse_mode: Option<ParseMode>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageText {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageText",
            &self,
        )
        .await
    }
}

impl EditMessageText {
    pub(crate) fn new<T>(
        bot: Arc<Bot>,
        chat_or_inline_message: ChatOrInlineMessage,
        text: T,
    ) -> Self
    where
        T: Into<String>,
    {
        Self {
            bot,
            chat_or_inline_message,
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// New text of the message.
    pub fn text<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.text = val.into();
        self
    }

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in your bot's message.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    /// Disables link previews for links in this message.
    pub fn disable_web_page_preview(mut self, val: bool) -> Self {
        self.disable_web_page_preview = Some(val);
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
