use serde::{Deserialize, Serialize};

use crate::types::{ShippingAddress, User};

/// This object contains information about an incoming shipping query.
///
/// [The official docs](https://core.telegram.org/bots/api#shippingquery).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: String,

    /// User who sent the query.
    pub from: User,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// User specified shipping address.
    pub shipping_address: ShippingAddress,
}

impl ShippingQuery {
    pub fn new<S1, S2>(
        id: S1,
        from: User,
        invoice_payload: S2,
        shipping_address: ShippingAddress,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self { id: id.into(), from, invoice_payload: invoice_payload.into(), shipping_address }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn from<S>(mut self, val: User) -> Self {
        self.from = val;
        self
    }

    pub fn invoice_payload<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.invoice_payload = val.into();
        self
    }

    pub fn shipping_address<S>(mut self, val: ShippingAddress) -> Self {
        self.shipping_address = val;
        self
    }
}
