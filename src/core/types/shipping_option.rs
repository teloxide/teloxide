use serde::Deserialize;

use crate::core::types::LabeledPrice;

#[derive(Debug, Deserialize)]
pub struct ShippingOption {
    pub id: i64,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
