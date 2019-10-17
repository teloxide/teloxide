use serde::Serialize;

use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};

#[derive(Debug, Clone, Serialize)]
/// Use this method to send point on the map. On success, the sent [`Message`]
/// is returned.
pub struct SendLocation<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Latitude of the location
    latitude: f64,
    /// Longitude of the location
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Period in seconds for which the location will be updated
    /// (see [Live Locations](https://telegram.org/blog/live-locations)),
    /// should be between 60 and 86400.
    live_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with
    /// no sound.
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendLocation<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendLocation<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendLocation",
            &self,
        )
        .await
    }
}

impl<'a> SendLocation<'a> {
    pub(crate) fn new<Lt, Lg, C>(
        bot: &'a Bot,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
    ) -> Self
    where
        Lt: Into<f64>,
        Lg: Into<f64>,
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude: latitude.into(),
            longitude: longitude.into(),
            live_period: None,
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

    pub fn latitude<Lt>(mut self, value: Lt) -> Self
    where
        Lt: Into<f64>,
    {
        self.latitude = value.into();
        self
    }

    pub fn longitude<Lg>(mut self, value: Lg) -> Self
    where
        Lg: Into<f64>,
    {
        self.longitude = value.into();
        self
    }

    pub fn live_period<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.live_period = Some(value.into());
        self
    }

    pub fn disable_notification<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(value.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(value.into());
        self
    }
}
