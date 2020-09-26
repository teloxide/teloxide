use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, ParseMode, True},
    Bot,
};

/// Use this method to edit captions of messages sent via the bot.
///
/// On success, [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagecaption).
///
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditInlineMessageCaption {
    #[serde(skip_serializing)]
    bot: Bot,
    inline_message_id: String,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditInlineMessageCaption {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(self.bot.client(), self.bot.token(), "editMessageCaption", &self).await
    }
}

impl EditInlineMessageCaption {
    pub(crate) fn new<I>(bot: Bot, inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self { bot, inline_message_id, caption: None, parse_mode: None, reply_markup: None }
    }

    /// Identifier of the inline message.
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
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
