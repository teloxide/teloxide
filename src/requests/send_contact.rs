use async_trait::async_trait;

use crate::{
    network,
    requests::{Request,  ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};
use crate::bot::Bot;

/// Use this method to send phone contacts.
/// returned.
#[derive(Debug, Clone, Serialize)]
pub struct SendContact<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a
    /// vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a
    /// notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove
    /// or ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendContact<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendContact<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendContact",
            &self,
        )
        .await
    }
}

impl<'a> SendContact<'a> {
    pub(crate) fn new<C, P, F>(
        bot: &'a Bot,
        chat_id: C,
        phone_number: P,
        first_name: F,
    ) -> Self
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
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

    pub fn phone_number<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.phone_number = value.into();
        self
    }

    pub fn first_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.first_name = value.into();
        self
    }

    pub fn last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    pub fn vcard<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(value.into());
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

    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}
