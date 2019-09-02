use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LabeledPrice {
    label: String,
    amount: i64,
}
