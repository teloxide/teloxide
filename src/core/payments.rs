use serde::Deserialize;
use crate::core::other::User;




#[derive(Debug, Deserialize)]
pub struct SendInvoice {
    chat_id: i64,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    start_parameter: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Deserialize)]
pub struct AnswerShippingQuery {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct  AnswerPreCheckoutQuery {
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LabeledPrice {
    label: String,
    amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    name: String,
    phone_number: String,
    email: String,
    shipping_address: ShippingAddress
}

#[derive(Debug, Deserialize)]
pub struct ShippingOption {
    id: i64,
    title: String,
    prices: Vec<LabeledPrice>,
}

#[derive(Debug, Deserialize)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Debug, Deserialize)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>
}