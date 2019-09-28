use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::{Message, ReplyMarkup},
};

#[derive(Debug, Clone, Serialize)]
/// Use this method to edit live location messages. A location can be edited
/// until its live_period expires or editing is explicitly disabled by a
/// call to [`stopMessageLiveLocation`]. On success, if the edited message
/// was sent by the bot, the edited [`Message`] is returned, otherwise True
/// is returned.
pub struct EditMessageLiveLocation<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat or username of the target channel (in the format
    /// @channelusername)
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the
    /// message to edit
    message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of
    /// the inline message
    inline_message_id: Option<String>,
    /// Latitude of new location
    latitude: f64,
    /// Longitude of new location
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for EditMessageLiveLocation<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl EditMessageLiveLocation<'_> {
    async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "editMessageLiveLocation",
            &self,
        )
        .await
    }
}

impl<'a> EditMessageLiveLocation<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            ctx,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, chat_id: T) -> Self {
        self.chat_id = Some(chat_id.into());
        self
    }

    pub fn message_id<T: Into<i32>>(mut self, message_id: T) -> Self {
        self.message_id = Some(message_id.into());
        self
    }

    pub fn inline_message_id<T>(mut self, inline_message_id: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = Some(inline_message_id.into());
        self
    }

    pub fn latitude<T: Into<f64>>(mut self, latitude: T) -> Self {
        self.latitude = latitude.into();
        self
    }

    pub fn longitude<T: Into<f64>>(mut self, longitude: T) -> Self {
        self.longitude = longitude.into();
        self
    }
}
