use crate::types::CountryCode;
use serde::{Deserialize, Serialize};

/// This object represents a shipping address.
///
/// [The official docs](https://core.telegram.org/bots/api#shippingaddress).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code.
    pub country_code: CountryCode,

    /// State, if applicable.
    pub state: String,

    /// City.
    pub city: String,

    /// First line for the address.
    pub street_line1: String,

    /// Second line for the address.
    pub street_line2: String,

    /// Address post code.
    pub post_code: String,
}
