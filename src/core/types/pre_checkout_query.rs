use serde::Deserialize;
use crate::core::types::{User, OrderInfo};

#[derive(Debug, Deserialize)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}
