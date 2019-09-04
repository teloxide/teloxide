#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}
