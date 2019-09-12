#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct AnswerPreCheckoutQuery {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
