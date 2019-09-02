use serde::Deserialize;
use crate::core::types::ShippingOption;

#[derive(Debug, Deserialize)]
pub struct AnswerShippingQuery {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}
