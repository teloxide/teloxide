use crate::core::types::LabeledPrice;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ShippingOption {
    pub id: i64,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
