use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ParseMode, InlineKeyboardMarkup, Message},
};

/// Use this method to edit captions of messages. On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct EditMessageCaptionInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// New caption of the message
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageCaptionInline {
    type Output = Message;

    const NAME: &'static str = "editMessageCaption";
}

impl json::Payload for EditMessageCaptionInline {}

impl dynamic::Payload for EditMessageCaptionInline {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageCaptionInline {
    pub fn new<I>(inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageCaptionInline> {
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.inline_message_id = val.into();
        self
    }

    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.payload.parse_mode = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
