use serde::Deserialize;

use crate::core::types::ShippingOption;

#[derive(Debug, Deserialize)]
pub struct AnswerShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>,
}
