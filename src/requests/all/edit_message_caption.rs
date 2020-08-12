use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

/// Use this method to edit captions of messages sent by the bot.
///
/// On success, the edited [`Message`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagecaption).
///
/// [`Message`]: crate::types::Message
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    message_id: i32,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageCaption {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "editMessageCaption", &self).await
    }
}

impl EditMessageCaption {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, message_id, caption: None, parse_mode: None, reply_markup: None }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`)
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Identifier of the message to edit
    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
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
