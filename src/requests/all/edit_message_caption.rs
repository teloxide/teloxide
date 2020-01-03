use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

/// Use this method to edit captions of messages. On success, if edited message
/// is sent by the bot, the edited Message is returned, otherwise True is
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaption<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,

    /// New caption of the message
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for EditMessageCaption<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageCaption",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> EditMessageCaption<'a> {
    pub(crate) fn new(
        bot: &'a Bot,
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

    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
