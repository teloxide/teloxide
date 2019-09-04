use crate::core::types::ShippingOption;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct AnswerShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
