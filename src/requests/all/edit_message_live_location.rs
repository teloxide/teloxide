use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to edit live location messages. A location can be edited
/// until its live_period expires or editing is explicitly disabled by a call to
/// stopMessageLiveLocation. On success, if the edited message was sent by the
/// bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageLiveLocation<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message to edit
    message_id: i32,
    /// Latitude of new location
    latitude: f32,
    /// Longitude of new location
    longitude: f32,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for EditMessageLiveLocation<'_> {
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

impl<'a> EditMessageLiveLocation<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        message_id: i32,
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
            message_id,
            latitude,
            longitude,
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

    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
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
