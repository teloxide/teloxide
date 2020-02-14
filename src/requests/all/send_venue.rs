use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};
use std::sync::Arc;

/// Use this method to send information about a venue.
///
/// [The official docs](https://core.telegram.org/bots/api#sendvenue).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendVenue {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    latitude: f32,
    longitude: f32,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendVenue {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendVenue",
            &self,
        )
        .await
    }
}

impl SendVenue {
    pub(crate) fn new<C, T, A>(
        bot: Arc<Bot>,
        chat_id: C,
        latitude: f32,
        longitude: f32,
        title: T,
        address: A,
    ) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        let chat_id = chat_id.into();
        let title = title.into();
        let address = address.into();
        Self {
            bot,
            chat_id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
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

    /// Latitude of the venue.
    pub fn latitude(mut self, val: f32) -> Self {
        self.latitude = val;
        self
    }

    /// Longitude of the venue.
    pub fn longitude(mut self, val: f32) -> Self {
        self.longitude = val;
        self
    }

    /// Name of the venue.
    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
        self
    }

    /// Address of the venue.
    pub fn address<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.address = val.into();
        self
    }

    /// Foursquare identifier of the venue.
    pub fn foursquare_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_id = Some(val.into());
        self
    }

    /// Foursquare type of the venue, if known.
    ///
    /// For example, `arts_entertainment/default`, `arts_entertainment/aquarium`
    /// or `food/icecream`.
    pub fn foursquare_type<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_type = Some(val.into());
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

    /// Additional interface options.
    ///
    /// A JSON-serialized object for an [inline keyboard], [custom reply
    /// keyboard], instructions to remove reply keyboard or to force a reply
    /// from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
