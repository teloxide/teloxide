use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{InlineKeyboardMarkup, LabeledPrice, Message},
    Bot,
};

/// Use this method to send invoices.
///
/// [The official docs](https://core.telegram.org/bots/api#sendinvoice).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendInvoice {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: i32,
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

#[async_trait::async_trait]
impl RequestOld for SendInvoice {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "sendInvoice", &self).await
    }
}

impl SendInvoice {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new<T, D, Pl, Pt, S, C, Pr>(
        bot: Bot,
        chat_id: i32,
        title: T,
        description: D,
        payload: Pl,
        provider_token: Pt,
        start_parameter: S,
        currency: C,
        prices: Pr,
    ) -> Self
    where
        T: Into<String>,
        D: Into<String>,
        Pl: Into<String>,
        Pt: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pr: Into<Vec<LabeledPrice>>,
    {
        let title = title.into();
        let description = description.into();
        let payload = payload.into();
        let provider_token = provider_token.into();
        let start_parameter = start_parameter.into();
        let currency = currency.into();
        let prices = prices.into();
        Self {
            bot,
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

    /// Unique identifier for the target private chat.
    pub fn chat_id(mut self, val: i32) -> Self {
        self.chat_id = val;
        self
    }

    /// Product name, 1-32 characters.
    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
        self
    }

    /// Product description, 1-255 characters.
    pub fn description<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.description = val.into();
        self
    }

    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to
    /// the user, use for your internal processes.
    pub fn payload<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.payload = val.into();
        self
    }

    /// Payments provider token, obtained via [@Botfather].
    ///
    /// [@Botfather]: https://t.me/botfather
    pub fn provider_token<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_token = val.into();
        self
    }

    /// Unique deep-linking parameter that can be used to generate this invoice
    /// when used as a start parameter.
    pub fn start_parameter<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.start_parameter = val.into();
        self
    }

    /// Three-letter ISO 4217 currency code, see [more on currencies].
    ///
    /// [more on currencies]: https://core.telegram.org/bots/payments#supported-currencies
    pub fn currency<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.currency = val.into();
        self
    }

    /// Price breakdown, a list of components (e.g. product price, tax,
    /// discount, delivery cost, delivery tax, bonus, etc.).
    pub fn prices<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<LabeledPrice>>,
    {
        self.prices = val.into();
        self
    }

    /// JSON-encoded data about the invoice, which will be shared with the
    /// payment provider.
    ///
    /// A detailed description of required fields should be provided by the
    /// payment provider.
    pub fn provider_data<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_data = Some(val.into());
        self
    }

    /// URL of the product photo for the invoice.
    ///
    /// Can be a photo of the goods or a marketing image for a service. People
    /// like it better when they see what they are paying for.
    pub fn photo_url<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.photo_url = Some(val.into());
        self
    }

    /// Photo size.
    pub fn photo_size(mut self, val: i32) -> Self {
        self.photo_size = Some(val);
        self
    }

    /// Photo width.
    pub fn photo_width(mut self, val: i32) -> Self {
        self.photo_width = Some(val);
        self
    }

    /// Photo height.
    pub fn photo_height(mut self, val: i32) -> Self {
        self.photo_height = Some(val);
        self
    }

    /// Pass `true`, if you require the user's full name to complete the order.
    pub fn need_name(mut self, val: bool) -> Self {
        self.need_name = Some(val);
        self
    }

    /// Pass `true`, if you require the user's phone number to complete the
    /// order.
    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.need_phone_number = Some(val);
        self
    }

    /// Pass `true`, if you require the user's email address to complete the
    /// order.
    pub fn need_email(mut self, val: bool) -> Self {
        self.need_email = Some(val);
        self
    }

    /// Pass `true`, if you require the user's shipping address to complete the
    /// order.
    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.need_shipping_address = Some(val);
        self
    }

    /// Pass `true`, if user's phone number should be sent to provider.
    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.send_phone_number_to_provider = Some(val);
        self
    }

    /// Pass `true`, if user's email address should be sent to provider.
    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.send_email_to_provider = Some(val);
        self
    }

    /// Pass `true`, if the final price depends on the shipping method.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.is_flexible = Some(val);
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

    /// A JSON-serialized object for an [inline keyboard].
    ///
    /// If empty, one 'Pay `total price`' button will be shown. If not empty,
    /// the first button must be a Pay button.
    ///
    /// [inlint keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
