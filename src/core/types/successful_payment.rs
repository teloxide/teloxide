use serde::Deserialize;
use crate::core::types::OrderInfo;

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
