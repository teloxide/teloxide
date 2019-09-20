use crate::types::{ShippingAddress, User};

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
