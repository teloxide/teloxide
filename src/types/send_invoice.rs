use serde::{Deserialize, Serialize};

use crate::types::{ChatId, InlineKeyboardMarkup, LabeledPrice};

// TODO: missing docs
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SendInvoice {
    pub chat_id: ChatId,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub start_parameter: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i32>,
    pub photo_width: Option<i32>,
    pub photo_height: Option<i32>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendInvoice {
    #[allow(clippy::too_many_arguments)]
    pub fn new<C, S1, S2, S3, S4, S5, S6, P>(
        chat_id: C,
        title: S1,
        description: S2,
        payload: S3,
        provider_token: S4,
        start_parameter: S5,
        currency: S6,
        prices: P,
    ) -> Self
    where
        C: Into<ChatId>,
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
        S6: Into<String>,
        P: Into<Vec<LabeledPrice>>,
    {
        Self {
            chat_id: chat_id.into(),
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            start_parameter: start_parameter.into(),
            currency: currency.into(),
            prices: prices.into(),
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<C>(mut self, val: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = val.into();
        self
    }

    pub fn payload<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.payload = val.into();
        self
    }

    pub fn provider_token<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.provider_token = val.into();
        self
    }

    pub fn start_parameter<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.start_parameter = val.into();
        self
    }

    pub fn currency<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.currency = val.into();
        self
    }

    pub fn prices<P>(mut self, val: P) -> Self
    where
        P: Into<Vec<LabeledPrice>>,
    {
        self.prices = val.into();
        self
    }

    pub fn provider_data<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.provider_data = Some(val.into());
        self
    }

    pub fn photo_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.photo_url = Some(val.into());
        self
    }

    pub fn photo_size(mut self, val: i32) -> Self {
        self.photo_size = Some(val);
        self
    }

    pub fn photo_width(mut self, val: i32) -> Self {
        self.photo_width = Some(val);
        self
    }

    pub fn photo_height(mut self, val: i32) -> Self {
        self.photo_height = Some(val);
        self
    }

    pub fn need_name(mut self, val: bool) -> Self {
        self.need_name = Some(val);
        self
    }

    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.need_phone_number = Some(val);
        self
    }

    pub fn need_email(mut self, val: bool) -> Self {
        self.need_email = Some(val);
        self
    }

    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.need_shipping_address = Some(val);
        self
    }

    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.send_phone_number_to_provider = Some(val);
        self
    }

    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.send_email_to_provider = Some(val);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.is_flexible = Some(val);
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
