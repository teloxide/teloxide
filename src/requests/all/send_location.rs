use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send point on the map. On success, the sent Message is
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendLocation<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Latitude of the location
    latitude: f32,
    /// Longitude of the location
    longitude: f32,
    /// Period in seconds for which the location will be updated (see Live
    /// Locations, should be between 60 and 86400.
    live_period: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard
    /// or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for SendLocation<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendLocation",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SendLocation<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        latitude: f32,
        longitude: f32,
    ) -> Self
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

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn latitude(mut self, val: f32) -> Self {
        self.latitude = val;
        self
    }

    pub fn longitude(mut self, val: f32) -> Self {
        self.longitude = val;
        self
    }

    pub fn live_period(mut self, val: i64) -> Self {
        self.live_period = Some(val);
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
