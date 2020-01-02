use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ParseMode, ReplyMarkup},
    Bot,
};

/// Use this method to send text messages.
///
/// On success, the sent [`Message`] is returned.
///
/// [`Message`]: crate::types::Message
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendMessage<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    ///	Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Text of the message to be sent
    pub text: String,
    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in your bot's message.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message silently.
    /// Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, [id] of the original message
    ///
    /// [id]: crate::types::Message::id
    pub reply_to_message_id: Option<i32>,
    /// Additional interface options.
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for SendMessage<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendMessage",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SendMessage<'a> {
    pub(crate) fn new<C, T>(bot: &'a Bot, chat_id: C, text: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn text<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.text = value.into();
        self
    }

    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self
    }

    pub fn disable_web_page_preview(mut self, value: bool) -> Self {
        self.disable_web_page_preview = Some(value);
        self
    }

    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }

    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}
