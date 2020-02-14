use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, Message},
    Bot,
};
use std::sync::Arc;

/// Use this method to edit live location messages.
///
/// A location can be edited until its live_period expires or editing is
/// explicitly disabled by a call to stopMessageLiveLocation. On success, if the
/// edited message was sent by the bot, the edited [`Message`] is returned,
/// otherwise [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagelivelocation).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,
    latitude: f32,
    longitude: f32,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageLiveLocation {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageLiveLocation",
            &self,
        )
        .await
    }
}

impl EditMessageLiveLocation {
    pub(crate) fn new(
        bot: Arc<Bot>,
        chat_or_inline_message: ChatOrInlineMessage,
        latitude: f32,
        longitude: f32,
    ) -> Self {
        Self {
            bot,
            chat_or_inline_message,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// Latitude of new location.
    pub fn latitude(mut self, val: f32) -> Self {
        self.latitude = val;
        self
    }

    /// Longitude of new location.
    pub fn longitude(mut self, val: f32) -> Self {
        self.longitude = val;
        self
    }

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
