use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{LabeledPrice, InlineKeyboardMarkup, Message},
};

/// Use this method to send invoices. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    chat_id: i32,
    /// Product name, 1-32 characters
    title: String,
    /// Product description, 1-255 characters
    description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    payload: String,
    /// Payments provider token, obtained via Botfather
    provider_token: String,
    /// Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter
    start_parameter: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    currency: String,
    /// Price breakdown, a list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    prices: Vec<LabeledPrice>,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    photo_url: Option<String>,
    /// Photo size
    photo_size: Option<i32>,
    /// Photo width
    photo_width: Option<i32>,
    /// Photo height
    photo_height: Option<i32>,
    /// Pass True, if you require the user's full name to complete the order
    need_name: Option<bool>,
    /// Pass True, if you require the user's phone number to complete the order
    need_phone_number: Option<bool>,
    /// Pass True, if you require the user's email address to complete the order
    need_email: Option<bool>,
    /// Pass True, if you require the user's shipping address to complete the order
    need_shipping_address: Option<bool>,
    /// Pass True, if user's phone number should be sent to provider
    send_phone_number_to_provider: Option<bool>,
    /// Pass True, if user's email address should be sent to provider
    send_email_to_provider: Option<bool>,
    /// Pass True, if the final price depends on the shipping method
    is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for SendInvoice {
    type Output = Message;

    const NAME: &'static str = "sendInvoice";
}

impl json::Payload for SendInvoice {}

impl dynamic::Payload for SendInvoice {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SendInvoice {
    #[allow(clippy::too_many_arguments)]
    pub fn new<T, D, Pl, Pt, S, C, Pr>(
        chat_id: i32,
        title: T,
        description: D,
        payload: Pl,
        provider_token: Pt,
        start_parameter: S,
        currency: C,
        prices: Pr
    ) -> Self
    where
        T: Into<String>,
        D: Into<String>,
        Pl: Into<String>,
        Pt: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pr: Into<Vec<LabeledPrice>>
    {
        let title = title.into();
        let description = description.into();
        let payload = payload.into();
        let provider_token = provider_token.into();
        let start_parameter = start_parameter.into();
        let currency = currency.into();
        let prices = prices.into();
        Self {
            chat_id,
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
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
}

impl json::Request<'_, SendInvoice> {
    pub fn chat_id(mut self, val: i32) -> Self {
        self.payload.chat_id = val;
        self
    }

    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.title = val.into();
        self
    }

    pub fn description<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.description = val.into();
        self
    }

    pub fn payload<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.payload = val.into();
        self
    }

    pub fn provider_token<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.provider_token = val.into();
        self
    }

    pub fn start_parameter<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.start_parameter = val.into();
        self
    }

    pub fn currency<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.currency = val.into();
        self
    }

    pub fn prices<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<LabeledPrice>>
    {
        self.payload.prices = val.into();
        self
    }

    pub fn provider_data<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.provider_data = Some(val.into());
        self
    }

    pub fn photo_url<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.photo_url = Some(val.into());
        self
    }

    pub fn photo_size(mut self, val: i32) -> Self {
        self.payload.photo_size = Some(val);
        self
    }

    pub fn photo_width(mut self, val: i32) -> Self {
        self.payload.photo_width = Some(val);
        self
    }

    pub fn photo_height(mut self, val: i32) -> Self {
        self.payload.photo_height = Some(val);
        self
    }

    pub fn need_name(mut self, val: bool) -> Self {
        self.payload.need_name = Some(val);
        self
    }

    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.payload.need_phone_number = Some(val);
        self
    }

    pub fn need_email(mut self, val: bool) -> Self {
        self.payload.need_email = Some(val);
        self
    }

    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.payload.need_shipping_address = Some(val);
        self
    }

    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.payload.send_phone_number_to_provider = Some(val);
        self
    }

    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.payload.send_email_to_provider = Some(val);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.payload.is_flexible = Some(val);
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.payload.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.payload.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
                 