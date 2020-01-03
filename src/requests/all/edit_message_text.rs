use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

/// Use this method to edit text and game messages. On success, if edited
/// message is sent by the bot, the edited Message is returned, otherwise True
/// is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageText<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,

    /// New text of the message
    text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    parse_mode: Option<ParseMode>,
    /// Disables link previews for links in this message
    disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageText<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageText",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> EditMessageText<'a> {
    pub(crate) fn new<T>(
        bot: &'a Bot,
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

    pub fn text<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.text = val.into();
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn disable_web_page_preview(mut self, val: bool) -> Self {
        self.disable_web_page_preview = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
