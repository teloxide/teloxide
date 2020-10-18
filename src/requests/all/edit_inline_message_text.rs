use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

/// Use this method to edit text and game messages sent via the bot.
///
/// On success, [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagetext).
///
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditInlineMessageText {
    #[serde(skip_serializing)]
    bot: Bot,
    inline_message_id: String,
    text: String,
    parse_mode: Option<ParseMode>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditInlineMessageText {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "editMessageText", &self).await
    }
}

impl EditInlineMessageText {
    pub(crate) fn new<I, T>(bot: Bot, inline_message_id: I, text: T) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        let text = text.into();
        Self {
            bot,
            inline_message_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Identifier of the inline message.
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
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
