use std::borrow::Cow;

use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::{Message, ReplyMarkup},
};

/// Use this method to send phone contacts.
/// returned.
#[derive(Debug, Clone, Serialize)]
pub struct SendContact<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatId<'a>,
    /// Contact's phone number
    pub phone_number: Cow<'a, str>,
    /// Contact's first name
    pub first_name: Cow<'a, str>,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Cow<'a, str>>,
    /// Additional data about the contact in the form of a
    /// vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<Cow<'a, str>>,
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
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

#[async_trait]
impl Request for SendContact<'_> {
    type ReturnValue = Message<'static>;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendContact<'_> {
    pub async fn send(self) -> ResponseResult<Message<'static>> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendContact",
            &self,
        )
        .await
    }
}

impl<'a> SendContact<'a> {
    pub(crate) fn new<C, S>(
        ctx: RequestContext<'a>,
        chat_id: C,
        phone_number: S,
        first_name: S,
    ) -> Self
    where
        C: Into<ChatId<'a>>,
        S: Into<Cow<'a, str>>,
    {
        Self {
            ctx,
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

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId<'a>>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn phone_number<T>(mut self, phone_number: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.phone_number = phone_number.into();
        self
    }

    pub fn first_name<T>(mut self, first_name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.first_name = first_name.into();
        self
    }

    pub fn last_name<T>(mut self, last_name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn vcard<T>(mut self, vcard: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.vcard = Some(vcard.into());
        self
    }

    pub fn disable_notification<T>(mut self, disable_notification: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, reply_to_message_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }

    pub fn reply_markup<T>(mut self, reply_markup: T) -> Self
    where
        T: Into<ReplyMarkup<'a>>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
