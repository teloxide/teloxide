use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i64,
}
