use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to edit live location messages. A location can be edited
/// until its live_period expires or editing is explicitly disabled by a call to
/// stopMessageLiveLocation. On success, if the edited message was sent by the
/// bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageLiveLocationInline<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Identifier of the inline message
    inline_message_id: String,
    /// Latitude of new location
    latitude: f32,
    /// Longitude of new location
    longitude: f32,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for EditMessageLiveLocationInline<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageLiveLocation",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> EditMessageLiveLocationInline<'a> {
    pub(crate) fn new<I>(
        bot: &'a Bot,
        inline_message_id: I,
        latitude: f32,
        longitude: f32,
    ) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            bot,
            inline_message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
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

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
