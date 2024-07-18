use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

/// This object represents information about an order.
///
/// [The official docs](https://core.telegram.org/bots/api#orderinfo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderInfo {
    /// User's name.
    pub name: Option<String>,

    /// User's phone number.
    pub phone_number: Option<String>,

    /// User's email.
    pub email: Option<String>,

    /// User's shipping address.
    pub shipping_address: Option<ShippingAddress>,
}
