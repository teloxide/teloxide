use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, True},
    Bot,
};

/// Use this method to edit live location messages sent via the bot.
///
/// A location can be edited until its live_period expires or editing is
/// explicitly disabled by a call to stopMessageLiveLocation. On success,
/// [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagelivelocation).
///
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditInlineMessageLiveLocation {
    #[serde(skip_serializing)]
    bot: Bot,
    inline_message_id: String,
    latitude: f32,
    longitude: f32,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditInlineMessageLiveLocation {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(self.bot.client(), self.bot.token(), "editMessageLiveLocation", &self)
            .await
    }
}

impl EditInlineMessageLiveLocation {
    pub(crate) fn new<I>(bot: Bot, inline_message_id: I, latitude: f32, longitude: f32) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self { bot, inline_message_id, latitude, longitude, reply_markup: None }
    }

    /// Identifier of the inline message.
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
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
