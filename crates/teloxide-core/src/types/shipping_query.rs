use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use crate::types::{ShippingAddress, User};

/// Unique query identifier.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct ShippingQueryId(pub String);

/// This object contains information about an incoming shipping query.
///
/// [The official docs](https://core.telegram.org/bots/api#shippingquery).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: ShippingQueryId,

    /// User who sent the query.
    pub from: User,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// User specified shipping address.
    pub shipping_address: ShippingAddress,
}
