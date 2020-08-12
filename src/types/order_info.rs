use serde::{Deserialize, Serialize};

use crate::types::ShippingAddress;

/// This object represents information about an order.
///
/// [The official docs](https://core.telegram.org/bots/api#orderinfo).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl OrderInfo {
    pub fn new<S1, S2, S3>(
        name: S1,
        phone_number: S2,
        email: S3,
        shipping_address: ShippingAddress,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            name: name.into(),
            phone_number: phone_number.into(),
            email: email.into(),
            shipping_address,
        }
    }

    pub fn name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.name = val.into();
        self
    }

    pub fn phone_number<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    pub fn email<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.email = val.into();
        self
    }

    pub fn shipping_address(mut self, val: ShippingAddress) -> Self {
        self.shipping_address = val;
        self
    }
}
