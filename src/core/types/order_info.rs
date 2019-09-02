use serde::Deserialize;
use crate::core::types::ShippingAddress;

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    name: String,
    phone_number: String,
    email: String,
    shipping_address: ShippingAddress,
}
