use crate::types::CountryCode;
use serde::{Deserialize, Serialize};

/// This object represents a shipping address.
///
/// [The official docs](https://core.telegram.org/bots/api#shippingaddress).
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

impl ShippingAddress {
    pub fn new<S1, S2, S3, S4, S5>(
        country_code: CountryCode,

        state: S1,
        city: S2,
        street_line1: S3,
        street_line2: S4,
        post_code: S5,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
    {
        Self {
            country_code,
            state: state.into(),
            city: city.into(),
            street_line1: street_line1.into(),
            street_line2: street_line2.into(),
            post_code: post_code.into(),
        }
    }

    pub fn country_code(mut self, val: CountryCode) -> Self {
        self.country_code = val;
        self
    }

    pub fn state<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.state = val.into();
        self
    }

    pub fn city<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.city = val.into();
        self
    }

    pub fn street_line1<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.street_line1 = val.into();
        self
    }

    pub fn street_line2<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.street_line2 = val.into();
        self
    }

    pub fn post_code<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.post_code = val.into();
        self
    }
}
