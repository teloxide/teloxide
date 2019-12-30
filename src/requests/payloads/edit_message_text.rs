use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, ParseMode, InlineKeyboardMarkup, Message},
};

/// Use this method to edit text and game messages. On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct EditMessageText {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message to edit
    message_id: i32,
    /// New text of the message
    text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    parse_mode: Option<ParseMode>,
    /// Disables link previews for links in this message
    disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageText {
    type Output = Message;

    const NAME: &'static str = "editMessageTextInline";
}

impl json::Payload for EditMessageText {}

impl dynamic::Payload for EditMessageText {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageText {
    pub fn new<C, T>(chat_id: C, message_id: i32, text: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        let chat_id = chat_id.into();
        let text = text.into();
        Self {
            chat_id,
            message_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageText> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.payload.message_id = val;
        self
    }

    pub fn text<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.text = val.into();
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.payload.parse_mode = Some(val);
        self
    }

    pub fn disable_web_page_preview(mut self, val: bool) -> Self {
        self.payload.disable_web_page_preview = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
