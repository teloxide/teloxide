use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message, ParseMode},
};

/// Use this method to edit text and game messages. On success, if edited
/// message is sent by the bot, the edited Message is returned, otherwise True
/// is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct EditMessageTextInline {
    /// Identifier of the inline message
    inline_message_id: String,
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
impl Request<Message> for EditMessageTextInline {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<Message> {
        network::request_json(
            bot.client(),
            bot.token(),
            "editMessageText",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl EditMessageTextInline {
    pub fn new<I, T>(inline_message_id: I, text: T) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        let text = text.into();
        Self {
            inline_message_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
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
