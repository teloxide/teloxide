use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

/// Use this method to edit captions of messages. On success, if edited message
/// is sent by the bot, the edited Message is returned, otherwise True is
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaptionInline<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Identifier of the inline message
    inline_message_id: String,
    /// New caption of the message
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for EditMessageCaptionInline<'_> {
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

impl<'a> EditMessageCaptionInline<'a> {
    pub(crate) fn new<I>(bot: &'a Bot, inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            bot,
            inline_message_id,
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
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
