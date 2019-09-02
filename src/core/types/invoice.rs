use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}
