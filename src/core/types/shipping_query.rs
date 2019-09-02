use serde::Deserialize;
use crate::core::types::{User, ShippingAddress};

#[derive(Debug, Deserialize)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}
