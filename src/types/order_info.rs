use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

/// This object represents information about an order.
///
/// [The official docs](https://core.telegram.org/bots/api#orderinfo).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OrderInfo {
    /// User's name.
    pub name: String,

    /// User's phone number.
    pub phone_number: String,

    /// User's email.
    pub email: String,

    /// User's shipping address.
    pub shipping_address: ShippingAddress,
}
