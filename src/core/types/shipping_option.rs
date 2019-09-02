use serde::Deserialize;
use crate::core::types::LabeledPrice;

#[derive(Debug, Deserialize)]
pub struct ShippingOption {
    id: i64,
    title: String,
    prices: Vec<LabeledPrice>,
}
