use serde::Deserialize;

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
    need_name: Option<Bool>,
    need_phone_number: Option<Bool>,
    need_email: Option<Bool>,
    need_shipping_address: Option<Bool>,
    send_phone_number_to_provider: Option<Bool>,
    send_email_to_provider: Option<Bool>,
    is_flexible: Option<Bool>,
    disable_notification: Option<Bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
}