use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send point on the map.
///
/// [The official docs](https://core.telegram.org/bots/api#sendlocation).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendLocation {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
    pub latitude: f32,
    pub longitude: f32,
    pub live_period: Option<i64>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendLocation {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "sendLocation", &self).await
    }
}

impl SendLocation {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, latitude: f32, longitude: f32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Latitude of the location.
    pub fn latitude(mut self, val: f32) -> Self {
        self.latitude = val;
        self
    }

    /// Longitude of the location.
    pub fn longitude(mut self, val: f32) -> Self {
        self.longitude = val;
        self
    }

    /// Period in seconds for which the location will be updated (see [Live
    /// Locations], should be between 60 and 86400).
    ///
    /// [Live Locations]: https://telegram.org/blog/live-locations
    pub fn live_period(mut self, val: i64) -> Self {
        self.live_period = Some(val);
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// A JSON-serialized object for an [inline keyboard].
    ///
    /// If empty, one 'Pay `total price`' button will be shown. If not empty,
    /// the first button must be a Pay button.
    ///
    /// [inlint keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
