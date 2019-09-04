#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}
